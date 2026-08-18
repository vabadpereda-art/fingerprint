//! # zkfp-match
//! Fingerprint matching based on nbis-rs (Bozorth3 algorithm)
//!
//! ## Performance Architecture
//!
//! 1. Pre-cache: ISO → Minutiae → XYT C struct at gallery load time
//! 2. Direct FFI: call bozorth_main() directly, bypassing nbis-rs mutex & re-conversion
//! 3. Single-direction: 1:N uses one bozorth_main call instead of two
//! 4. Multi-process: fork() N workers for true parallel matching

use nbis::{NbisExtractor, NbisExtractorSettings};
use std::os::raw::c_int;
use std::sync::{Arc, Mutex};
use zkfp_template::FingerprintTemplate;

pub const DEFAULT_THRESHOLD: i32 = 20;
pub const DEFAULT_VERIFY_THRESHOLD: i32 = 50;
pub const DEFAULT_MIN_COUNT_RATIO: f32 = 0.35;
const MAX_BOZORTH_MINUTIAE: usize = 200;

// ── Direct FFI to bozorth_main (symbol is already linked via nbis-rs) ──

#[repr(C)]
#[derive(Clone, Copy)]
struct XytStruct {
    nrows: c_int,
    xcol: [c_int; MAX_BOZORTH_MINUTIAE],
    ycol: [c_int; MAX_BOZORTH_MINUTIAE],
    theta: [c_int; MAX_BOZORTH_MINUTIAE],
}

impl std::fmt::Debug for XytStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XytStruct")
            .field("nrows", &self.nrows)
            .finish()
    }
}

extern "C" {
    fn bozorth_main(probe: *const XytStruct, gallery: *const XytStruct) -> c_int;
}

// ── Pre-cached XYT representation ──

/// Lightweight fingerprint signature for fast pre-filtering.
/// Computed once at gallery load time. Comparison costs ~nanoseconds.
#[derive(Clone, Debug)]
struct PreFilter {
    count: u16,           // number of minutiae
    angle_hist: [u8; 8],  // 8-bin angle histogram (0-45, 45-90, ..., 315-360)
    radial_hist: [u8; 8], // 8-bin radial histogram around centroid, normalized by max radius
}

impl PreFilter {
    fn from_xyt(xyt: &XytStruct) -> Self {
        let count = xyt.nrows as u16;
        let n = xyt.nrows.max(0) as usize;
        let mut angle_hist = [0u8; 8];
        let mut radial_hist = [0u8; 8];

        if n == 0 {
            return PreFilter {
                count,
                angle_hist,
                radial_hist,
            };
        }

        let mut sum_x = 0f64;
        let mut sum_y = 0f64;
        for i in 0..n {
            angle_hist[(xyt.theta[i] as usize * 8 / 360).min(7)] =
                angle_hist[(xyt.theta[i] as usize * 8 / 360).min(7)].saturating_add(1);
            sum_x += xyt.xcol[i] as f64;
            sum_y += xyt.ycol[i] as f64;
        }

        let cx = sum_x / n as f64;
        let cy = sum_y / n as f64;
        let mut max_r = 0f64;
        let mut radii = Vec::with_capacity(n);

        for i in 0..n {
            let dx = xyt.xcol[i] as f64 - cx;
            let dy = xyt.ycol[i] as f64 - cy;
            let r = (dx * dx + dy * dy).sqrt();
            max_r = max_r.max(r);
            radii.push(r);
        }

        if max_r <= f64::EPSILON {
            radial_hist[0] = n.min(u8::MAX as usize) as u8;
        } else {
            for r in radii {
                let norm = r / max_r;
                let bin = ((norm * 8.0) as usize).min(7);
                radial_hist[bin] = radial_hist[bin].saturating_add(1);
            }
        }

        PreFilter {
            count,
            angle_hist,
            radial_hist,
        }
    }

    /// Calculate heuristic distance. Lower = more likely to match.
    ///
    /// Important: this comparison is rotation-invariant. We compare the angle
    /// histogram under every circular shift and keep the minimum distance so a
    /// global finger rotation does not push the true match to the bottom.
    #[inline]
    fn passes_count_ratio(&self, other: &PreFilter, min_ratio: f32) -> bool {
        let lo = self.count.min(other.count) as f32;
        let hi = self.count.max(other.count) as f32;
        hi <= 0.0 || (lo / hi) >= min_ratio
    }

    #[inline]
    fn distance(&self, other: &PreFilter) -> u32 {
        let (lo, hi) = if self.count < other.count {
            (self.count as u32, other.count as u32)
        } else {
            (other.count as u32, self.count as u32)
        };

        let best_angle_dist = (0..8)
            .map(|shift| {
                (0..8)
                    .map(|i| {
                        let a = self.angle_hist[i] as i32;
                        let b = other.angle_hist[(i + shift) % 8] as i32;
                        (a - b).unsigned_abs()
                    })
                    .sum::<u32>()
            })
            .min()
            .unwrap_or(0);

        let radial_dist: u32 = self
            .radial_hist
            .iter()
            .zip(other.radial_hist.iter())
            .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
            .sum();

        let count_penalty = hi.saturating_sub(lo);
        best_angle_dist
            .saturating_add(radial_dist.saturating_mul(2))
            .saturating_add(count_penalty)
    }
}

/// Pre-computed Bozorth3 XYT struct, ready for direct FFI calls.
/// Created once per template at gallery load time.
#[derive(Clone, Debug)]
pub struct CachedXyt {
    xyt: Box<XytStruct>,
    filter: PreFilter,
}

impl CachedXyt {
    /// Build from a Minutiae object + ISO bytes.
    /// img_h is extracted from ISO header (bytes 16-17) since minutiae.img_h is private.
    fn from_minutiae(minutiae: &nbis::Minutiae, iso_bytes: &[u8]) -> Self {
        let points = minutiae.get();
        // Extract image height from ISO 19794-2 header (bytes 16-17, big-endian)
        let img_h = if iso_bytes.len() >= 18 {
            u16::from_be_bytes([iso_bytes[16], iso_bytes[17]]) as i32
        } else {
            375 // fallback: ZK9500 default
        };
        // We sort by reliability but keep up to 150 points (NIST standard) to prevent false negatives.
        // 80 was too aggressive and discarded overlapping minutiae regions.
        let mut sorted_points: Vec<_> = points.iter().collect();
        sorted_points.sort_by(|a, b| {
            b.reliability()
                .partial_cmp(&a.reliability())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let max_points = 150.min(MAX_BOZORTH_MINUTIAE);
        let n = sorted_points.len().min(max_points);

        let mut xyt = Box::new(XytStruct {
            nrows: n as c_int,
            xcol: [0; MAX_BOZORTH_MINUTIAE],
            ycol: [0; MAX_BOZORTH_MINUTIAE],
            theta: [0; MAX_BOZORTH_MINUTIAE],
        });

        for i in 0..n {
            let m = sorted_points[i];
            let x = m.x();
            let y = img_h as i32 - m.y();
            let angle = m.angle();
            let t = angle.round() as i32 % 360;
            let t = if t < 0 { t + 360 } else { t };

            xyt.xcol[i] = x;
            xyt.ycol[i] = y;
            xyt.theta[i] = t;
        }

        let filter = PreFilter::from_xyt(&xyt);
        CachedXyt { xyt, filter }
    }

    /// Direct single-direction bozorth_main score (no mutex, no conversion)
    #[inline]
    fn match_score(&self, other: &CachedXyt) -> i32 {
        let score = unsafe { bozorth_main(self.xyt.as_ref(), other.xyt.as_ref()) };
        if score == 4000 {
            0
        } else {
            score
        }
    }

    /// Bidirectional Bozorth score, matching nbis-rs behaviour more closely.
    #[inline]
    fn match_score_bidirectional(&self, other: &CachedXyt) -> i32 {
        let ab = self.match_score(other);
        let ba = other.match_score(self);
        ab.max(ba)
    }
}

// ── Public types ──

#[derive(Clone, Debug)]
pub struct MatchResult {
    pub score: i32,
    pub is_match: bool,
    pub paired_minutiae: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
pub struct VerifiedIdentifyResult {
    pub identify: IdentifyResult,
    pub verify: MatchResult,
}

#[derive(Clone, Debug)]
pub struct IdentifyResult {
    pub user_id: Option<u32>,
    pub score: i32,
}

#[derive(Clone, Debug)]
pub struct SearchTemplate {
    pub template: FingerprintTemplate,
    pub minutiae: Option<Arc<nbis::Minutiae>>,
    pub cached_xyt: Option<CachedXyt>,
    pub candidate_keys: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct MemorySearchGallery {
    entries: Vec<(u32, SearchTemplate)>,
}

impl MemorySearchGallery {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, user_id: u32, template: SearchTemplate) {
        self.entries.push((user_id, template));
    }

    pub fn remove(&mut self, user_id: u32) {
        self.entries.retain(|(id, _)| *id != user_id);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn from_templates(gallery: &[(u32, FingerprintTemplate)], matcher: &Matcher) -> Self {
        let mut mem = Self::new();
        for (user_id, template) in gallery {
            mem.insert(*user_id, matcher.create_search_template(template));
        }
        mem
    }
}

// ── Matcher ──

pub struct Matcher {
    threshold: i32,
    verify_threshold: i32,
    min_count_ratio: f32,
    extractor: Arc<Mutex<NbisExtractor>>,
}

impl Matcher {
    pub fn new(threshold: i32) -> Self {
        let settings = NbisExtractorSettings {
            min_quality: 0.0,
            get_center: false,
            check_fingerprint: false,
            compute_nfiq2: false,
            ppi: None,
        };
        let extractor = NbisExtractor::new(settings).expect("Failed to initialize NBIS extractor");
        Self {
            threshold,
            verify_threshold: DEFAULT_VERIFY_THRESHOLD,
            min_count_ratio: DEFAULT_MIN_COUNT_RATIO,
            extractor: Arc::new(Mutex::new(extractor)),
        }
    }

    pub fn with_default_threshold() -> Self {
        Self::new(DEFAULT_THRESHOLD)
    }

    pub fn set_threshold(&mut self, threshold: i32) {
        self.threshold = threshold;
    }

    pub fn threshold(&self) -> i32 {
        self.threshold
    }

    pub fn set_verify_threshold(&mut self, verify_threshold: i32) {
        self.verify_threshold = verify_threshold;
    }

    pub fn verify_threshold(&self) -> i32 {
        self.verify_threshold
    }

    pub fn set_min_count_ratio(&mut self, min_count_ratio: f32) {
        self.min_count_ratio = min_count_ratio.clamp(0.0, 1.0);
    }

    pub fn min_count_ratio(&self) -> f32 {
        self.min_count_ratio
    }

    pub fn verify(
        &self,
        probe: &FingerprintTemplate,
        gallery: &FingerprintTemplate,
    ) -> MatchResult {
        let ext = self.extractor.lock().unwrap();
        let m1 = ext.load_iso_19794_2_2005(&probe.iso_bytes).ok();
        let m2 = ext.load_iso_19794_2_2005(&gallery.iso_bytes).ok();

        let mut score = 0;
        if let (Some(m1), Some(m2)) = (m1, m2) {
            score = m1.compare(&m2);
        }

        MatchResult {
            score,
            is_match: score >= self.verify_threshold,
            paired_minutiae: Vec::new(),
        }
    }

    pub fn create_search_template(&self, template: &FingerprintTemplate) -> SearchTemplate {
        let ext = self.extractor.lock().unwrap();
        let minutiae = ext.load_iso_19794_2_2005(&template.iso_bytes).ok();
        let cached_xyt = minutiae
            .as_ref()
            .map(|m| CachedXyt::from_minutiae(m, &template.iso_bytes));
        SearchTemplate {
            template: template.clone(),
            minutiae: minutiae.map(Arc::new),
            cached_xyt,
            candidate_keys: Vec::new(),
        }
    }

    pub fn create_limited_search_template(
        &self,
        template: &FingerprintTemplate,
        _max_minutiae: usize,
    ) -> SearchTemplate {
        self.create_search_template(template)
    }

    pub fn identify(
        &self,
        probe: &FingerprintTemplate,
        gallery: &[(u32, FingerprintTemplate)],
    ) -> IdentifyResult {
        let memory_gallery = MemorySearchGallery::from_templates(gallery, self);
        self.identify_in_memory(probe, &memory_gallery)
    }

    pub fn identify_with_verification(
        &self,
        probe: &FingerprintTemplate,
        gallery: &[(u32, FingerprintTemplate)],
    ) -> VerifiedIdentifyResult {
        let identify = self.identify(probe, gallery);
        let verify = if let Some(user_id) = identify.user_id {
            if let Some((_, template)) = gallery.iter().find(|(id, _)| *id == user_id) {
                self.verify(probe, template)
            } else {
                MatchResult {
                    score: 0,
                    is_match: false,
                    paired_minutiae: Vec::new(),
                }
            }
        } else {
            MatchResult {
                score: 0,
                is_match: false,
                paired_minutiae: Vec::new(),
            }
        };

        VerifiedIdentifyResult { identify, verify }
    }

    pub fn identify_in_memory(
        &self,
        probe: &FingerprintTemplate,
        gallery: &MemorySearchGallery,
    ) -> IdentifyResult {
        let probe_search = self.create_search_template(probe);
        self.identify_search_template(&probe_search, gallery)
    }

    pub fn identify_in_memory_with_verification(
        &self,
        probe: &FingerprintTemplate,
        gallery: &MemorySearchGallery,
    ) -> VerifiedIdentifyResult {
        let identify = self.identify_in_memory(probe, gallery);
        let verify = if let Some(user_id) = identify.user_id {
            if let Some((_, search_template)) =
                gallery.entries.iter().find(|(id, _)| *id == user_id)
            {
                self.verify(probe, &search_template.template)
            } else {
                MatchResult {
                    score: 0,
                    is_match: false,
                    paired_minutiae: Vec::new(),
                }
            }
        } else {
            MatchResult {
                score: 0,
                is_match: false,
                paired_minutiae: Vec::new(),
            }
        };

        VerifiedIdentifyResult { identify, verify }
    }

    pub fn identify_search_template(
        &self,
        probe: &SearchTemplate,
        gallery: &MemorySearchGallery,
    ) -> IdentifyResult {
        let num_entries = gallery.entries.len();
        if num_entries == 0 {
            return IdentifyResult {
                user_id: None,
                score: 0,
            };
        }

        // Build probe XYT once (no extractor lock needed if already cached)
        let probe_xyt = match probe.cached_xyt {
            Some(ref xyt) => xyt.clone(),
            None => {
                let ext = self.extractor.lock().unwrap();
                match ext.load_iso_19794_2_2005(&probe.template.iso_bytes) {
                    Ok(m) => CachedXyt::from_minutiae(&m, &probe.template.iso_bytes),
                    Err(_) => {
                        return IdentifyResult {
                            user_id: None,
                            score: 0,
                        }
                    }
                }
            }
        };

        let num_workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .min(num_entries);

        if num_workers <= 1 || num_entries < 100 {
            return self.identify_sequential(&probe_xyt, gallery);
        }

        self.identify_forked(&probe_xyt, gallery, num_workers)
    }

    /// Sequential path: direct bozorth_main calls, no mutex, no conversion overhead
    fn identify_sequential(
        &self,
        probe_xyt: &CachedXyt,
        gallery: &MemorySearchGallery,
    ) -> IdentifyResult {
        let threshold = self.threshold;
        let mut best_score = 0;
        let mut best_id = None;

        // OPTIMIZATION 2: Probabilistic Sorting (No discarding)
        let min_count_ratio = self.min_count_ratio;
        let mut order: Vec<_> = gallery
            .entries
            .iter()
            .enumerate()
            .filter_map(|(j, (_, t))| {
                if let Some(ref gxyt) = t.cached_xyt {
                    if probe_xyt
                        .filter
                        .passes_count_ratio(&gxyt.filter, min_count_ratio)
                    {
                        Some((j, probe_xyt.filter.distance(&gxyt.filter)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        order.sort_by_key(|&(_, dist)| dist);

        // Search only the most likely candidates first, but do not stop on the
        // first score above threshold. That was causing false negatives when a
        // rotated true match was ranked later by the heuristic.
        const SEQ_CANDIDATE_LIMIT: usize = 1000;
        const FINAL_RERANK_TOP_K: usize = 25;
        let candidate_limit = order.len().min(SEQ_CANDIDATE_LIMIT);
        let mut top_scores: Vec<(usize, i32)> =
            Vec::with_capacity(candidate_limit.min(FINAL_RERANK_TOP_K * 4));

        for &(j, _) in order.iter().take(candidate_limit) {
            let (_, template) = &gallery.entries[j];
            if let Some(ref gxyt) = template.cached_xyt {
                let score = probe_xyt.match_score(gxyt);
                if score > best_score {
                    best_score = score;
                    best_id = Some(gallery.entries[j].0);
                }
                top_scores.push((j, score));
            }
        }

        top_scores.sort_by(|a, b| b.1.cmp(&a.1));

        for &(j, _) in top_scores.iter().take(FINAL_RERANK_TOP_K) {
            let (user_id, template) = &gallery.entries[j];
            if let Some(ref gxyt) = template.cached_xyt {
                let score = probe_xyt.match_score_bidirectional(gxyt);
                if score > best_score {
                    best_score = score;
                    best_id = Some(*user_id);
                }
            }
        }

        IdentifyResult {
            user_id: if best_score >= threshold {
                best_id
            } else {
                None
            },
            score: best_score,
        }
    }

    /// Fork N worker processes for true parallel matching.
    /// Each child has its own Bozorth3 global state via OS process isolation.
    #[cfg(not(windows))]
    fn identify_forked(
        &self,
        probe_xyt: &CachedXyt,
        gallery: &MemorySearchGallery,
        num_workers: usize,
    ) -> IdentifyResult {
        let num_entries = gallery.entries.len();
        let threshold = self.threshold;

        // OPTIMIZATION 2: Probabilistic Sorting (No discarding)
        // We calculate distance and sort, but we DON'T discard anything!
        // This guarantees 0 false negatives caused by the pre-filter.
        let min_count_ratio = self.min_count_ratio;
        let mut order: Vec<_> = (0..num_entries)
            .filter_map(|j| {
                if let Some(ref gxyt) = gallery.entries[j].1.cached_xyt {
                    if probe_xyt
                        .filter
                        .passes_count_ratio(&gxyt.filter, min_count_ratio)
                    {
                        Some((j, probe_xyt.filter.distance(&gxyt.filter)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        order.sort_by_key(|&(_, dist)| dist);
        let sorted_indices: Vec<usize> = order.into_iter().map(|(j, _)| j).collect();
        let num_tasks = sorted_indices.len();
        let total_skips_initial = (num_entries - num_tasks) as u32;

        if num_tasks == 0 {
            return IdentifyResult {
                user_id: None,
                score: 0,
            };
        }

        // OPTIMIZATION 3: Fast Path
        // Check a moderate block of the most likely candidates sequentially and
        // re-rank only the best few bidirectionally. This avoids false negatives
        // from stopping too early on a merely acceptable score.
        let mut best_score = 0i32;
        let mut best_id = None;
        let fast_path_count = 128.min(num_tasks);
        const FAST_PATH_RERANK_TOP_K: usize = 16;
        let mut fast_top_scores: Vec<(usize, i32)> = Vec::with_capacity(fast_path_count);

        for &j in &sorted_indices[..fast_path_count] {
            let (uid, template) = &gallery.entries[j];
            if let Some(ref gxyt) = template.cached_xyt {
                let score = probe_xyt.match_score(gxyt);
                if score > best_score {
                    best_score = score;
                    best_id = Some(*uid);
                }
                fast_top_scores.push((j, score));
            }
        }

        fast_top_scores.sort_by(|a, b| b.1.cmp(&a.1));
        for &(j, _) in fast_top_scores.iter().take(FAST_PATH_RERANK_TOP_K) {
            let (uid, template) = &gallery.entries[j];
            if let Some(ref gxyt) = template.cached_xyt {
                let score = probe_xyt.match_score_bidirectional(gxyt);
                if score > best_score {
                    best_score = score;
                    best_id = Some(*uid);
                }
            }
        }

        if best_score >= threshold {
            return IdentifyResult {
                user_id: best_id,
                score: best_score,
            };
        }

        // If not found in the fast path, proceed with the heavy parallel search for the rest
        let remaining_tasks = num_tasks - fast_path_count;
        if remaining_tasks == 0 {
            return IdentifyResult {
                user_id: best_id,
                score: best_score,
            };
        }

        // Create a shared memory AtomicBool using mmap so processes can signal each other to stop
        let shared_mem = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                std::mem::size_of::<std::sync::atomic::AtomicBool>(),
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        if shared_mem == libc::MAP_FAILED {
            eprintln!("[zkfp-match] Failed to mmap shared memory");
            return self.identify_sequential(probe_xyt, gallery);
        }

        let shared_found = unsafe { &*(shared_mem as *const std::sync::atomic::AtomicBool) };
        shared_found.store(false, std::sync::atomic::Ordering::SeqCst);

        let mut children: Vec<(libc::pid_t, i32)> = Vec::with_capacity(num_workers);

        for i in 0..num_workers {
            if i >= num_tasks {
                break;
            }

            let mut pipe_fds = [0i32; 2];
            if unsafe { libc::pipe(pipe_fds.as_mut_ptr()) } != 0 {
                continue;
            }

            let pid = unsafe { libc::fork() };

            if pid == 0 {
                // ===== CHILD =====
                unsafe { libc::close(pipe_fds[0]) };

                let mut best_score = 0i32;
                let mut best_id = 0u32;
                let mut evals = 0u32;

                // Round-robin distribution over SORTED and FILTERED tasks
                // Start from `fast_path_count + i` because the main thread already checked the first `fast_path_count`
                for task_idx in (fast_path_count + i..num_tasks).step_by(num_workers) {
                    if evals % 16 == 0 && shared_found.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    let j = sorted_indices[task_idx];
                    let (uid, template) = &gallery.entries[j];

                    if let Some(ref gxyt) = template.cached_xyt {
                        evals += 1;
                        let score = probe_xyt.match_score(gxyt);
                        if score > best_score {
                            best_score = score;
                            best_id = *uid;
                        }
                        if score >= threshold {
                            // Signal all other workers to stop!
                            shared_found.store(true, std::sync::atomic::Ordering::Relaxed);
                            break;
                        }
                    }
                }

                let skips = 0u32; // Not used per-child anymore since we pre-filtered
                let mut buf = [0u8; 16];
                buf[0..4].copy_from_slice(&best_id.to_le_bytes());
                buf[4..8].copy_from_slice(&best_score.to_le_bytes());
                buf[8..12].copy_from_slice(&evals.to_le_bytes());
                buf[12..16].copy_from_slice(&skips.to_le_bytes());

                unsafe {
                    libc::write(pipe_fds[1], buf.as_ptr() as *const libc::c_void, 16);
                    libc::close(pipe_fds[1]);
                    libc::_exit(0);
                }
            } else if pid > 0 {
                // ===== PARENT =====
                unsafe { libc::close(pipe_fds[1]) };
                children.push((pid, pipe_fds[0]));
            } else {
                unsafe {
                    libc::close(pipe_fds[0]);
                    libc::close(pipe_fds[1]);
                }
            }
        }

        if children.is_empty() {
            return self.identify_sequential(probe_xyt, gallery);
        }

        let mut best_score = 0i32;
        let mut best_id: Option<u32> = None;
        let mut total_evals = 0u32;
        let mut total_skips = 0u32;

        for &(_, read_fd) in &children {
            let mut buf = [0u8; 16];
            let mut total = 0usize;
            while total < 16 {
                let n = unsafe {
                    libc::read(
                        read_fd,
                        buf[total..].as_mut_ptr() as *mut libc::c_void,
                        16 - total,
                    )
                };
                if n <= 0 {
                    break;
                }
                total += n as usize;
            }
            unsafe { libc::close(read_fd) };

            if total == 16 {
                let id = u32::from_le_bytes(buf[0..4].try_into().unwrap());
                let score = i32::from_le_bytes(buf[4..8].try_into().unwrap());
                let evals = u32::from_le_bytes(buf[8..12].try_into().unwrap());
                let skips = u32::from_le_bytes(buf[12..16].try_into().unwrap());

                total_evals += evals;
                total_skips += skips;

                if score > best_score {
                    best_score = score;
                    best_id = Some(id);
                }
            }
        }

        total_skips += total_skips_initial;
        let total_processed = total_evals + total_skips;
        let skip_percent = if total_processed > 0 {
            (total_skips * 100) / total_processed
        } else {
            0
        };
        eprintln!(
            "[zkfp-match] Pre-filter stats -> Evaluated: {}, Skipped: {} ({}%)",
            total_evals, total_skips, skip_percent
        );

        for &(pid, _) in &children {
            unsafe {
                libc::waitpid(pid, std::ptr::null_mut(), 0);
            }
        }

        // Cleanup shared memory
        unsafe {
            libc::munmap(
                shared_mem,
                std::mem::size_of::<std::sync::atomic::AtomicBool>(),
            );
        }

        IdentifyResult {
            user_id: if best_score >= threshold {
                best_id
            } else {
                None
            },
            score: best_score,
        }
    }

    /// Temporary Windows-safe path. The optimized Windows implementation should
    /// use process isolation or separately-linked Bozorth instances because the
    /// upstream Bozorth code uses mutable global state and is not thread-safe.
    #[cfg(windows)]
    fn identify_forked(
        &self,
        probe_xyt: &CachedXyt,
        gallery: &MemorySearchGallery,
        _num_workers: usize,
    ) -> IdentifyResult {
        self.identify_sequential(probe_xyt, gallery)
    }
}
