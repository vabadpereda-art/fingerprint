[# Guías de uso del SDK `zkfp-capi`

Esta carpeta reúne guías prácticas para usar el SDK en distintas plataformas.

## Documentos disponibles

- `linux.md` — compilación, despliegue y uso en Linux
- `windows.md` — compilación, despliegue y uso en Windows
- `android.md` — integración orientativa en Android

## Qué cubren estas guías

- cómo compilar la librería nativa
- dónde colocar la librería compartida
- cómo enlazar headers o bindings
- cómo usar el SDK para:
  - captura y extracción de plantillas
  - matching en memoria
  - base de datos local offline
  - bulk load tipado
  - carga directa de gallery desde SQLite local
  - sincronización / reconstrucción de snapshot desde PostgreSQL

## Recomendación general

Para producción, el flujo recomendado es:

1. traer datos remotos desde PostgreSQL u otra fuente externa
2. reconstruir la DB local SQLite con bulk tipado
3. cargar la gallery directamente desde la DB local con `zkfp_gallery_load_from_db()`
4. operar la app contra la DB local y la memoria nativa

La referencia detallada de la API C sigue estando en:

- `crates/zkfp-capi/API.md`
](error: failed to run custom build command for `nbis-rs v0.1.3 (D:\a\fingerprint\fingerprint\crates\nbis-rs)`
Caused by:
  process didn't exit successfully: `D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-b1527ebcf4526dd3\build-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-env-changed=CLIPPY
  CMAKE_TOOLCHAIN_FILE_x86_64-pc-windows-msvc = None
  CMAKE_TOOLCHAIN_FILE_x86_64_pc_windows_msvc = None
  HOST_CMAKE_TOOLCHAIN_FILE = None
  CMAKE_TOOLCHAIN_FILE = None
  CMAKE_GENERATOR_x86_64-pc-windows-msvc = None
  CMAKE_GENERATOR_x86_64_pc_windows_msvc = None
  HOST_CMAKE_GENERATOR = None
  CMAKE_GENERATOR = None
  CMAKE_PREFIX_PATH_x86_64-pc-windows-msvc = None
  CMAKE_PREFIX_PATH_x86_64_pc_windows_msvc = None
  HOST_CMAKE_PREFIX_PATH = None
  CMAKE_PREFIX_PATH = None
  CMAKE_x86_64-pc-windows-msvc = None
  CMAKE_x86_64_pc_windows_msvc = None
  HOST_CMAKE = None
  CMAKE = None
  -- The C compiler identification is MSVC 19.44.35226.0
  -- The CXX compiler identification is MSVC 19.44.35226.0
  -- Detecting C compiler ABI info
  -- Detecting C compiler ABI info - done
  -- Check for working C compiler: C:/Program Files/Microsoft Visual Studio/2022/Enterprise/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/cl.exe - skipped
  -- Detecting C compile features
  -- Detecting C compile features - done
  -- Detecting CXX compiler ABI info
  -- Detecting CXX compiler ABI info - done
  -- Check for working CXX compiler: C:/Program Files/Microsoft Visual Studio/2022/Enterprise/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/cl.exe - skipped
  -- Detecting CXX compile features
  -- Detecting CXX compile features - done
  -- NFIQ 2 Superbuild
  -- Embedding random forest parameters
  -- Detected MSVC compiler (Windows)
  -- NFIQ2 git hash a8f8046
  -- Configuring done (24.6s)
  -- Generating done (0.1s)
  -- Build files have been written to: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build
  MSBuild version 17.14.40+3e7442088 for .NET Framework
    1>Checking Build System
    Creating directories for 'OpenCV'
    Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/CMakeLists.txt
    No download step for 'OpenCV'
    No update step for 'OpenCV'
    No patch step for 'OpenCV'
    Performing configure step for 'OpenCV'
    CMake Deprecation Warning at CMakeLists.txt:25 (cmake_minimum_required):
      Compatibility with CMake < 3.10 will be removed from a future version of
      CMake.
    
      Update the VERSION argument <min> value.  Or, use the <min>...<max> syntax
      to tell CMake that the project requires at least <min> but has been updated
      to work with policies introduced by <max> or earlier.
    
    
    -- 'Release' build type is used by default. Use CMAKE_BUILD_TYPE to specify build type (Release or Debug)
    CMake Warning (dev) at CMakeLists.txt:127 (enable_language):
      project() should be called prior to this enable_language() call.
    This warning is for project developers.  Use -Wno-dev to suppress it.
    
    -- The CXX compiler identification is MSVC 19.44.35226.0
    -- The C compiler identification is MSVC 19.44.35226.0
    CMake Warning (dev) at C:/Program Files/CMake/share/cmake-3.31/Modules/Platform/Windows-MSVC.cmake:529 (enable_language):
      project() should be called prior to this enable_language() call.
    Call Stack (most recent call first):
      C:/Program Files/CMake/share/cmake-3.31/Modules/Platform/Windows-MSVC.cmake:501 (__windows_compiler_msvc_enable_rc)
      C:/Program Files/CMake/share/cmake-3.31/Modules/Platform/Windows-MSVC-CXX.cmake:6 (__windows_compiler_msvc)
      C:/Program Files/CMake/share/cmake-3.31/Modules/CMakeCXXInformation.cmake:48 (include)
      CMakeLists.txt:127 (enable_language)
    This warning is for project developers.  Use -Wno-dev to suppress it.
    
    -- Detecting CXX compiler ABI info
    -- Detecting CXX compiler ABI info - done
    -- Check for working CXX compiler: C:/Program Files/Microsoft Visual Studio/2022/Enterprise/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/cl.exe - skipped
    -- Detecting CXX compile features
    -- Detecting CXX compile features - done
    CMake Warning (dev) at C:/Program Files/CMake/share/cmake-3.31/Modules/Platform/Windows-MSVC.cmake:529 (enable_language):
      project() should be called prior to this enable_language() call.
    Call Stack (most recent call first):
      C:/Program Files/CMake/share/cmake-3.31/Modules/Platform/Windows-MSVC.cmake:501 (__windows_compiler_msvc_enable_rc)
      C:/Program Files/CMake/share/cmake-3.31/Modules/Platform/Windows-MSVC-C.cmake:5 (__windows_compiler_msvc)
      C:/Program Files/CMake/share/cmake-3.31/Modules/CMakeCInformation.cmake:48 (include)
      CMakeLists.txt:127 (enable_language)
    This warning is for project developers.  Use -Wno-dev to suppress it.
    
    -- Detecting C compiler ABI info
    -- Detecting C compiler ABI info - done
    -- Check for working C compiler: C:/Program Files/Microsoft Visual Studio/2022/Enterprise/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/cl.exe - skipped
    -- Detecting C compile features
    -- Detecting C compile features - done
    -- Detected processor: AMD64
    -- Found PythonInterp: C:/hostedtoolcache/windows/Python/3.12.10/x64/python3.exe (found suitable version "3.12.10", minimum required is "3.2")
    -- Found PythonLibs: C:/hostedtoolcache/windows/Python/3.12.10/x64/libs/python312.lib (found suitable exact version "3.12.10")
    -- Performing Test HAVE_CXX_FP:PRECISE
    -- Performing Test HAVE_CXX_FP:PRECISE - Success
    -- Performing Test HAVE_C_FP:PRECISE
    -- Performing Test HAVE_C_FP:PRECISE - Success
    -- Performing Test HAVE_CPU_SSE3_SUPPORT (check file: cmake/checks/cpu_sse3.cpp)
    -- Performing Test HAVE_CPU_SSE3_SUPPORT - Success
    -- Performing Test HAVE_CPU_SSSE3_SUPPORT (check file: cmake/checks/cpu_ssse3.cpp)
    -- Performing Test HAVE_CPU_SSSE3_SUPPORT - Success
    -- Performing Test HAVE_CPU_SSE4_1_SUPPORT (check file: cmake/checks/cpu_sse41.cpp)
    -- Performing Test HAVE_CPU_SSE4_1_SUPPORT - Success
    -- Performing Test HAVE_CPU_POPCNT_SUPPORT (check file: cmake/checks/cpu_popcnt.cpp)
    -- Performing Test HAVE_CPU_POPCNT_SUPPORT - Success
    -- Performing Test HAVE_CPU_SSE4_2_SUPPORT (check file: cmake/checks/cpu_sse42.cpp)
    -- Performing Test HAVE_CPU_SSE4_2_SUPPORT - Success
    -- Performing Test HAVE_CXX_ARCH:AVX (check file: cmake/checks/cpu_fp16.cpp)
    -- Performing Test HAVE_CXX_ARCH:AVX - Success
    -- Performing Test HAVE_CXX_ARCH:AVX2 (check file: cmake/checks/cpu_avx2.cpp)
    -- Performing Test HAVE_CXX_ARCH:AVX2 - Success
    -- Performing Test HAVE_CXX_ARCH:AVX512 (check file: cmake/checks/cpu_avx512.cpp)
    -- Performing Test HAVE_CXX_ARCH:AVX512 - Success
    -- Performing Test HAVE_CPU_BASELINE_FLAGS
    -- Performing Test HAVE_CPU_BASELINE_FLAGS - Success
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_SSE4_1
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_SSE4_1 - Success
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_SSE4_2
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_SSE4_2 - Success
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_FP16
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_FP16 - Success
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_AVX
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_AVX - Success
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_AVX2
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_AVX2 - Success
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_AVX512_SKX
    -- Performing Test HAVE_CPU_DISPATCH_FLAGS_AVX512_SKX - Success
    -- Performing Test HAVE_CXX_W15240
    -- Performing Test HAVE_CXX_W15240 - Success
    -- Performing Test HAVE_C_W15240
    -- Performing Test HAVE_C_W15240 - Success
    -- Looking for malloc.h
    -- Looking for malloc.h - found
    -- Looking for _aligned_malloc
    -- Looking for _aligned_malloc - found
    -- Looking for fseeko
    -- Looking for fseeko - not found
    -- Looking for sys/types.h
    -- Looking for sys/types.h - found
    -- Looking for stdint.h
    -- Looking for stdint.h - found
    -- Looking for stddef.h
    -- Looking for stddef.h - found
    -- Check size of off64_t
    -- Check size of off64_t - failed
    -- Could not find OpenBLAS include. Turning OpenBLAS_FOUND off
    -- Could not find OpenBLAS lib. Turning OpenBLAS_FOUND off
    -- Looking for sgemm_
    -- Looking for sgemm_ - not found
    -- Performing Test CMAKE_HAVE_LIBC_PTHREAD
    -- Performing Test CMAKE_HAVE_LIBC_PTHREAD - Failed
    -- Looking for pthread_create in pthreads
    -- Looking for pthread_create in pthreads - not found
    -- Looking for pthread_create in pthread
    -- Looking for pthread_create in pthread - not found
    -- Found Threads: TRUE
    -- Could NOT find BLAS (missing: BLAS_LIBRARIES) 
    -- Could NOT find LAPACK (missing: LAPACK_LIBRARIES) 
        Reason given by package: LAPACK could not be found because dependency BLAS could not be found.
    
    -- Found Java: C:/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/17.0.19-10/x64/bin/java.exe (found version "17.0.19")
    -- Found JNI: C:/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/17.0.19-10/x64/include  found components: AWT JVM
    -- VTK is not found. Please set -DVTK_DIR in CMake to VTK build directory, or to VTK install subdirectory with VTKConfig.cmake file
    -- Using whitelist: opencv_core;opencv_imgcodecs;opencv_imgproc;opencv_ml
    -- Module opencv_calib3d disabled by whitelist
    -- Module opencv_dnn disabled by whitelist
    -- Module opencv_features2d disabled by whitelist
    -- Module opencv_flann disabled by whitelist
    -- Module opencv_highgui disabled by whitelist
    -- Module opencv_java_bindings_generator disabled by whitelist
    -- Module opencv_java disabled by whitelist
    -- Module opencv_js_bindings_generator disabled by whitelist
    -- Module opencv_objc_bindings_generator disabled by whitelist
    -- Module opencv_objdetect disabled by whitelist
    -- Module opencv_photo disabled by whitelist
    -- Module opencv_python_bindings_generator disabled by whitelist
    -- Module opencv_python_tests disabled by whitelist
    -- Module opencv_python3 disabled by whitelist
    -- Module opencv_stitching disabled by whitelist
    -- Module opencv_video disabled by whitelist
    -- Module opencv_videoio disabled by whitelist
    -- Allocator metrics storage type: 'long long'
    -- Excluding from source files list: modules/imgproc/src/imgwarp.lasx.cpp
    -- Excluding from source files list: modules/imgproc/src/resize.lasx.cpp
    CMake Warning (dev) at CMakeLists.txt:1155 (install):
      Policy CMP0177 is not set: install() DESTINATION paths are normalized.  Run
      "cmake --help-policy CMP0177" for policy details.  Use the cmake_policy
      command to set the policy and suppress this warning.
    This warning is for project developers.  Use -Wno-dev to suppress it.
    
    -- 
    -- General configuration for OpenCV 4.10.0 =====================================
    --   Version control:               a8f8046
    -- 
    --   Platform:
    --     Timestamp:                   2026-05-13T15:16:58Z
    --     Host:                        Windows 10.0.26100 AMD64
    --     CMake:                       3.31.6
    --     CMake generator:             Visual Studio 17 2022
    --     CMake build tool:            C:/Program Files/Microsoft Visual Studio/2022/Enterprise/MSBuild/Current/Bin/amd64/MSBuild.exe
    --     MSVC:                        1944
    --     Configuration:               RelWithDebInfo
    -- 
    --   CPU/HW features:
    --     Baseline:                    SSE SSE2 SSE3
    --       requested:                 SSE3
    --     Dispatched code generation:  SSE4_1 SSE4_2 FP16 AVX AVX2 AVX512_SKX
    --       requested:                 SSE4_1 SSE4_2 AVX FP16 AVX2 AVX512_SKX
    --       SSE4_1 (13 files):         + SSSE3 SSE4_1
    --       SSE4_2 (1 files):          + SSSE3 SSE4_1 POPCNT SSE4_2
    --       FP16 (0 files):            + SSSE3 SSE4_1 POPCNT SSE4_2 FP16 AVX
    --       AVX (3 files):             + SSSE3 SSE4_1 POPCNT SSE4_2 AVX
    --       AVX2 (25 files):           + SSSE3 SSE4_1 POPCNT SSE4_2 FP16 FMA3 AVX AVX2
    --       AVX512_SKX (2 files):      + SSSE3 SSE4_1 POPCNT SSE4_2 FP16 FMA3 AVX AVX2 AVX_512F AVX512_COMMON AVX512_SKX
    -- 
    --   C/C++:
    --     Built as dynamic libs?:      NO
    --     C++ standard:                11
    --     C++ Compiler:                C:/Program Files/Microsoft Visual Studio/2022/Enterprise/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/cl.exe  (ver 19.44.35226.0)
    --     C++ flags (Release):         /DWIN32 /D_WINDOWS /W4 /GR  /D _CRT_SECURE_NO_DEPRECATE /D _CRT_NONSTDC_NO_DEPRECATE /D _SCL_SECURE_NO_WARNINGS /Gy /bigobj /Oi  /fp:precise     /EHa /wd4127 /wd4251 /wd4324 /wd4275 /wd4512 /wd4589 /wd4819 /MP  /O2 /Ob2 /DNDEBUG 
    --     C++ flags (Debug):           /DWIN32 /D_WINDOWS /W4 /GR  /D _CRT_SECURE_NO_DEPRECATE /D _CRT_NONSTDC_NO_DEPRECATE /D _SCL_SECURE_NO_WARNINGS /Gy /bigobj /Oi  /fp:precise     /EHa /wd4127 /wd4251 /wd4324 /wd4275 /wd4512 /wd4589 /wd4819 /MP  /Zi /Ob0 /Od /RTC1 
    --     C Compiler:                  C:/Program Files/Microsoft Visual Studio/2022/Enterprise/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/cl.exe
    --     C flags (Release):           /DWIN32 /D_WINDOWS /W3  /D _CRT_SECURE_NO_DEPRECATE /D _CRT_NONSTDC_NO_DEPRECATE /D _SCL_SECURE_NO_WARNINGS /Gy /bigobj /Oi  /fp:precise     /MP   /O2 /Ob2 /DNDEBUG 
    --     C flags (Debug):             /DWIN32 /D_WINDOWS /W3  /D _CRT_SECURE_NO_DEPRECATE /D _CRT_NONSTDC_NO_DEPRECATE /D _SCL_SECURE_NO_WARNINGS /Gy /bigobj /Oi  /fp:precise     /MP /Zi /Ob0 /Od /RTC1 
    --     Linker flags (Release):      /machine:x64  /NODEFAULTLIB:atlthunk.lib /INCREMENTAL:NO  /NODEFAULTLIB:libcmtd.lib /NODEFAULTLIB:libcpmtd.lib /NODEFAULTLIB:msvcrtd.lib
    --     Linker flags (Debug):        /machine:x64  /NODEFAULTLIB:atlthunk.lib /debug /INCREMENTAL  /NODEFAULTLIB:libcmt.lib /NODEFAULTLIB:libcpmt.lib /NODEFAULTLIB:msvcrt.lib
    --     ccache:                      NO
    --     Precompiled headers:         NO
    --     Extra dependencies:
    --     3rdparty dependencies:       zlib
    -- 
    --   OpenCV modules:
    --     To be built:                 core imgcodecs imgproc ml
    --     Disabled:                    world
    --     Disabled by dependency:      calib3d dnn features2d flann highgui java java_bindings_generator js_bindings_generator objc_bindings_generator objdetect photo python3 python_bindings_generator python_tests stitching video videoio
    --     Unavailable:                 gapi python2 ts
    --     Applications:                -
    --     Documentation:               NO
    --     Non-free algorithms:         NO
    -- 
    --   Windows RT support:            NO
    -- 
    --   GUI: 
    --     Win32 UI:                    YES
    --     VTK support:                 NO
    -- 
    --   Media I/O: 
    --     ZLib:                        build (ver 1.3.1)
    --     HDR:                         YES
    --     SUNRASTER:                   YES
    --     PXM:                         YES
    --     PFM:                         YES
    -- 
    --   Video I/O:
    --     DC1394:                      NO
    --     GStreamer:                   NO
    -- 
    --   Parallel framework:            Concurrency
    -- 
    --   Trace:                         YES (built-in)
    -- 
    --   Other third-party libraries:
    --     Lapack:                      NO
    --     Custom HAL:                  NO
    --     Flatbuffers:                 builtin/3rdparty (23.5.9)
    -- 
    --   OpenCL:                        YES (NVD3D11)
    --     Include path:                D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/3rdparty/include/opencl/1.2
    --     Link libraries:              Dynamic load
    -- 
    --   Python 3:
    --     Interpreter:                 C:/hostedtoolcache/windows/Python/3.12.10/x64/python3.exe (ver 3.12.10)
    --     Libraries:                   NO
    --     Limited API:                 NO
    --     numpy:                       C:/hostedtoolcache/windows/Python/3.12.10/x64/Lib/site-packages/numpy/_core/include (ver 2.4.4)
    --     install path:                -
    -- 
    --   Python (for build):            C:/hostedtoolcache/windows/Python/3.12.10/x64/python3.exe
    -- 
    --   Java:                          
    --     ant:                         NO
    --     Java:                        YES (ver 17.0.19)
    --     JNI:                         C:/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/17.0.19-10/x64/include C:/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/17.0.19-10/x64/include/win32 C:/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/17.0.19-10/x64/include
    --     Java wrappers:               NO
    --     Java tests:                  NO
    -- 
    --   Install to:                    D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2
    -- -----------------------------------------------------------------
    -- 
    -- Configuring done (89.0s)
    -- Generating done (0.6s)
    -- Build files have been written to: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/OpenCV-prefix/src/OpenCV-build
    Performing build step for 'OpenCV'
    MSBuild version 17.14.40+3e7442088 for .NET Framework
    
      1>Checking Build System
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/core/CMakeLists.txt
      mathfuncs_core.avx.cpp
      opencv_core_AVX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_AVX.dir\RelWithDebInfo\opencv_core_AVX.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/core/CMakeLists.txt
      mathfuncs_core.avx2.cpp
      stat.avx2.cpp
      arithm.avx2.cpp
      convert.avx2.cpp
      convert_scale.avx2.cpp
      count_non_zero.avx2.cpp
      has_non_zero.avx2.cpp
      matmul.avx2.cpp
      mean.avx2.cpp
      merge.avx2.cpp
      split.avx2.cpp
      sum.avx2.cpp
      opencv_core_AVX2.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_AVX2.dir\RelWithDebInfo\opencv_core_AVX2.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/core/CMakeLists.txt
      matmul.avx512_skx.cpp
      opencv_core_AVX512_SKX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_AVX512_SKX.dir\RelWithDebInfo\opencv_core_AVX512_SKX.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/core/CMakeLists.txt
      arithm.sse4_1.cpp
      matmul.sse4_1.cpp
      opencv_core_SSE4_1.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_SSE4_1.dir\RelWithDebInfo\opencv_core_SSE4_1.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/core/CMakeLists.txt
      stat.sse4_2.cpp
      opencv_core_SSE4_2.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_SSE4_2.dir\RelWithDebInfo\opencv_core_SSE4_2.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/CMakeLists.txt
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/CMakeLists.txt
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/imgproc/CMakeLists.txt
      corner.avx.cpp
      accum.avx.cpp
      opencv_imgproc_AVX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_AVX.dir\RelWithDebInfo\opencv_imgproc_AVX.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/imgproc/CMakeLists.txt
      imgwarp.avx2.cpp
      resize.avx2.cpp
      accum.avx2.cpp
      bilateral_filter.avx2.cpp
      box_filter.avx2.cpp
      filter.avx2.cpp
      color_hsv.avx2.cpp
      color_rgb.avx2.cpp
      color_yuv.avx2.cpp
      median_blur.avx2.cpp
      morph.avx2.cpp
      smooth.avx2.cpp
      sumpixels.avx2.cpp
      opencv_imgproc_AVX2.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_AVX2.dir\RelWithDebInfo\opencv_imgproc_AVX2.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/imgproc/CMakeLists.txt
      sumpixels.avx512_skx.cpp
      opencv_imgproc_AVX512_SKX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_AVX512_SKX.dir\RelWithDebInfo\opencv_imgproc_AVX512_SKX.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/imgproc/CMakeLists.txt
      imgwarp.sse4_1.cpp
      resize.sse4_1.cpp
      accum.sse4_1.cpp
      box_filter.sse4_1.cpp
      filter.sse4_1.cpp
      color_hsv.sse4_1.cpp
      color_rgb.sse4_1.cpp
      color_yuv.sse4_1.cpp
      median_blur.sse4_1.cpp
      morph.sse4_1.cpp
      smooth.sse4_1.cpp
      opencv_imgproc_SSE4_1.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_SSE4_1.dir\RelWithDebInfo\opencv_imgproc_SSE4_1.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/videoio/CMakeLists.txt
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/3rdparty/zlib/CMakeLists.txt
      adler32.c
      compress.c
      crc32.c
      deflate.c
      gzclose.c
      gzlib.c
      gzread.c
      gzwrite.c
      inflate.c
      infback.c
      inftrees.c
      inffast.c
      trees.c
      uncompr.c
      zutil.c
      zlib.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\3rdparty\lib\RelWithDebInfo\zlib.lib
      Processing OpenCL kernels (core)
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/core/CMakeLists.txt
      algorithm.cpp
      arithm.cpp
      arithm.dispatch.cpp
      array.cpp
      async.cpp
      batch_distance.cpp
      bindings_utils.cpp
      buffer_area.cpp
      channels.cpp
      check.cpp
      command_line_parser.cpp
      conjugate_gradient.cpp
      convert.dispatch.cpp
      convert_c.cpp
      convert_scale.dispatch.cpp
      copy.cpp
      count_non_zero.dispatch.cpp
      cuda_gpu_mat.cpp
      cuda_gpu_mat_nd.cpp
      cuda_host_mem.cpp
      cuda_info.cpp
      cuda_stream.cpp
      datastructs.cpp
      directx.cpp
      downhill_simplex.cpp
      dxt.cpp
      gl_core_3_1.cpp
      glob.cpp
      hal_internal.cpp
      has_non_zero.dispatch.cpp
      kmeans.cpp
      lapack.cpp
      lda.cpp
      logger.cpp
      lpsolver.cpp
      lut.cpp
      mathfuncs.cpp
      mathfuncs_core.dispatch.cpp
      matmul.dispatch.cpp
      matrix.cpp
      matrix_c.cpp
      matrix_decomp.cpp
      matrix_expressions.cpp
      matrix_iterator.cpp
      matrix_operations.cpp
      matrix_sparse.cpp
      matrix_transform.cpp
      matrix_wrap.cpp
      mean.dispatch.cpp
      merge.dispatch.cpp
      minmax.cpp
      norm.cpp
      ocl.cpp
      opencl_clblas.cpp
      opencl_clfft.cpp
      opencl_core.cpp
      opengl.cpp
      out.cpp
      ovx.cpp
      parallel_openmp.cpp
      parallel_tbb.cpp
      parallel_impl.cpp
      pca.cpp
      persistence.cpp
      persistence_base64_encoding.cpp
      persistence_json.cpp
      persistence_types.cpp
      persistence_xml.cpp
      persistence_yml.cpp
      rand.cpp
      softfloat.cpp
      split.dispatch.cpp
      stat.dispatch.cpp
      stat_c.cpp
      stl.cpp
      sum.dispatch.cpp
      system.cpp
      tables.cpp
      trace.cpp
      types.cpp
      umatrix.cpp
      datafile.cpp
      filesystem.cpp
      logtagconfigparser.cpp
      logtagmanager.cpp
      samples.cpp
      va_intel.cpp
      opencl_kernels_core.cpp
      alloc.cpp
      parallel.cpp
      parallel.cpp
      opencv_core.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_core4100.lib
      Processing OpenCL kernels (imgproc)
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/imgproc/CMakeLists.txt
      accum.cpp
      accum.dispatch.cpp
      approx.cpp
      bilateral_filter.dispatch.cpp
      blend.cpp
      box_filter.dispatch.cpp
      canny.cpp
      clahe.cpp
      color.cpp
      color_hsv.dispatch.cpp
      color_lab.cpp
      color_rgb.dispatch.cpp
      color_yuv.dispatch.cpp
      colormap.cpp
      connectedcomponents.cpp
      contours.cpp
      contours_approx.cpp
      contours_common.cpp
      contours_link.cpp
      contours_new.cpp
      convhull.cpp
      corner.cpp
      cornersubpix.cpp
      demosaicing.cpp
      deriv.cpp
      distransform.cpp
      drawing.cpp
      emd.cpp
      emd_new.cpp
      featureselect.cpp
      filter.dispatch.cpp
      floodfill.cpp
      gabor.cpp
      generalized_hough.cpp
      geometry.cpp
      grabcut.cpp
      hershey_fonts.cpp
      histogram.cpp
      hough.cpp
      imgwarp.cpp
      intelligent_scissors.cpp
      intersection.cpp
      linefit.cpp
      lsd.cpp
      main.cpp
      matchcontours.cpp
      median_blur.dispatch.cpp
      min_enclosing_triangle.cpp
      moments.cpp
      morph.dispatch.cpp
      phasecorr.cpp
      pyramids.cpp
      resize.cpp
      rotcalipers.cpp
      samplers.cpp
      segmentation.cpp
      shapedescr.cpp
      smooth.dispatch.cpp
      spatialgradient.cpp
      stackblur.cpp
      subdivision2d.cpp
      sumpixels.dispatch.cpp
      tables.cpp
      templmatch.cpp
      thresh.cpp
      utils.cpp
      opencl_kernels_imgproc.cpp
      opencv_imgproc.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_imgproc4100.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/imgcodecs/CMakeLists.txt
      loadsave.cpp
      utils.cpp
      grfmt_avif.cpp
      grfmt_base.cpp
      grfmt_bmp.cpp
      grfmt_exr.cpp
      grfmt_gdal.cpp
      grfmt_gdcm.cpp
      grfmt_hdr.cpp
      grfmt_jpeg.cpp
      grfmt_jpeg2000.cpp
      grfmt_jpeg2000_openjpeg.cpp
      grfmt_pam.cpp
      grfmt_pfm.cpp
      grfmt_png.cpp
      grfmt_pxm.cpp
      grfmt_spng.cpp
      grfmt_sunras.cpp
      grfmt_tiff.cpp
      grfmt_webp.cpp
      bitstrm.cpp
      rgbe.cpp
      exif.cpp
      opencv_imgcodecs.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_imgcodecs4100.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/modules/ml/CMakeLists.txt
      ann_mlp.cpp
      boost.cpp
      data.cpp
      em.cpp
      gbt.cpp
      inner_functions.cpp
      kdtree.cpp
      knearest.cpp
      lr.cpp
      nbayes.cpp
      rtrees.cpp
      svm.cpp
      svmsgd.cpp
      testset.cpp
      tree.cpp
      opencv_ml.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_ml4100.lib
      Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/opencv/CMakeLists.txt
    Performing install step for 'OpenCV'
    MSBuild version 17.14.40+3e7442088 for .NET Framework
    
      opencv_core_AVX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_AVX.dir\RelWithDebInfo\opencv_core_AVX.lib
      opencv_core_AVX2.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_AVX2.dir\RelWithDebInfo\opencv_core_AVX2.lib
      opencv_core_AVX512_SKX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_AVX512_SKX.dir\RelWithDebInfo\opencv_core_AVX512_SKX.lib
      opencv_core_SSE4_1.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_SSE4_1.dir\RelWithDebInfo\opencv_core_SSE4_1.lib
      opencv_core_SSE4_2.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\core\opencv_core_SSE4_2.dir\RelWithDebInfo\opencv_core_SSE4_2.lib
      opencv_imgproc_AVX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_AVX.dir\RelWithDebInfo\opencv_imgproc_AVX.lib
      opencv_imgproc_AVX2.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_AVX2.dir\RelWithDebInfo\opencv_imgproc_AVX2.lib
      opencv_imgproc_AVX512_SKX.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_AVX512_SKX.dir\RelWithDebInfo\opencv_imgproc_AVX512_SKX.lib
      opencv_imgproc_SSE4_1.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\modules\imgproc\opencv_imgproc_SSE4_1.dir\RelWithDebInfo\opencv_imgproc_SSE4_1.lib
      zlib.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\3rdparty\lib\RelWithDebInfo\zlib.lib
      opencv_core.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_core4100.lib
      opencv_imgproc.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_imgproc4100.lib
      opencv_imgcodecs.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_imgcodecs4100.lib
      opencv_ml.vcxproj -> D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\OpenCV-prefix\src\OpenCV-build\lib\RelWithDebInfo\opencv_ml4100.lib
      1>
      -- Install configuration: "RelWithDebInfo"
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/licenses/flatbuffers-LICENSE.txt
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/licenses/opencl-headers-LICENSE.txt
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/cvconfig.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/opencv_modules.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/OpenCVModules.cmake
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/OpenCVModules-relwithdebinfo.cmake
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/OpenCVConfig-version.cmake
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/OpenCVConfig.cmake
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/./OpenCVConfig-version.cmake
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/./OpenCVConfig.cmake
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/./LICENSE
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/./setup_vars_opencv4.cmd
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/zlib.lib
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/licenses/zlib-LICENSE
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/opencv.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/opencv_core4100.lib
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/affine.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/async.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/base.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/bindings_utils.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/bufferpool.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/check.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/core.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/core_c.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/block.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/border_interpolate.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/color.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/common.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/datamov_utils.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/detail/color_detail.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/detail/reduce.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/detail/reduce_key_val.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/detail/transform_detail.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/detail/type_traits_detail.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/detail/vec_distance_detail.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/dynamic_smem.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/emulation.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/filters.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/funcattrib.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/functional.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/limits.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/reduce.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/saturate_cast.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/scan.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/simd_functions.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/transform.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/type_traits.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/utility.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/vec_distance.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/vec_math.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/vec_traits.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/warp.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/warp_reduce.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda/warp_shuffle.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda_stream_accessor.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cuda_types.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cv_cpu_dispatch.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cv_cpu_helper.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cvdef.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cvstd.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cvstd.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/cvstd_wrapper.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/detail/async_promise.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/detail/dispatch_helper.impl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/detail/exception_ptr.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/directx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/dualquaternion.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/dualquaternion.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/eigen.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/fast_math.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/hal.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/interface.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_avx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_avx512.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_cpp.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_forward.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_lasx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_lsx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_msa.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_neon.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv071.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv_010_compat_non-policy.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv_010_compat_overloaded-non-policy.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv_011_compat.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv_compat_overloaded.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_rvv_scalable.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_sse.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_sse_em.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_vsx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/intrin_wasm.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/msa_macros.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/hal/simd_utils.impl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/mat.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/mat.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/matx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/matx.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/neon_utils.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/ocl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/ocl_genbase.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/ocl_defs.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/opencl_info.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/opencl_svm.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/autogenerated/opencl_clblas.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/autogenerated/opencl_clfft.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/autogenerated/opencl_core.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/autogenerated/opencl_core_wrappers.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/autogenerated/opencl_gl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/autogenerated/opencl_gl_wrappers.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_clblas.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_clfft.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_core.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_core_wrappers.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_gl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_gl_wrappers.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_svm_20.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_svm_definitions.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opencl/runtime/opencl_svm_hsa_extension.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/opengl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/operations.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/optim.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/ovx.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/parallel/backend/parallel_for.openmp.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/parallel/backend/parallel_for.tbb.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/parallel/parallel_backend.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/persistence.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/quaternion.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/quaternion.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/saturate.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/simd_intrinsics.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/softfloat.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/sse_utils.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/traits.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/types.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/types_c.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utility.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/allocator_stats.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/allocator_stats.impl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/filesystem.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/fp_control_utils.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/instrumentation.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/logger.defines.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/logger.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/logtag.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/tls.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/utils/trace.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/va_intel.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/version.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/core/vsx_utils.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/licenses/SoftFloat-COPYING.txt
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/opencv_imgproc4100.lib
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/bindings.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/detail/gcgraph.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/detail/legacy.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/hal/hal.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/hal/interface.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/imgproc.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/imgproc_c.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/segmentation.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgproc/types_c.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/opencv_ml4100.lib
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/ml.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/ml/ml.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/ml/ml.inl.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/x64/vc17/staticlib/opencv_imgcodecs4100.lib
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgcodecs.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgcodecs/imgcodecs.hpp
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgcodecs/imgcodecs_c.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgcodecs/ios.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgcodecs/legacy/constants_c.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/include/opencv2/imgcodecs/macosx.h
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_eye.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_eye_tree_eyeglasses.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_frontalcatface.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_frontalcatface_extended.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_frontalface_alt.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_frontalface_alt2.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_frontalface_alt_tree.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_frontalface_default.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_fullbody.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_lefteye_2splits.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_license_plate_rus_16stages.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_lowerbody.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_profileface.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_righteye_2splits.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_russian_plate_number.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_smile.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/haarcascades/haarcascade_upperbody.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/lbpcascades/lbpcascade_frontalcatface.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/lbpcascades/lbpcascade_frontalface.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/lbpcascades/lbpcascade_frontalface_improved.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/lbpcascades/lbpcascade_profileface.xml
      -- Installing: D:/a/fingerprint/fingerprint/target/release/build/nbis-rs-27437145f5b106f6/out/build/install_staging/nfiq2/etc/lbpcascades/lbpcascade_silverware.xml
    Completed 'OpenCV'
    Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/CMakeLists.txt
    Building Custom Rule D:/a/fingerprint/fingerprint/crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/CMakeLists.txt
  cl : command line  warning D9002: ignoring unknown option '-std=c++11' [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    FRFXLLCreateContext.cpp
  cl : command line  warning D9002: ignoring unknown option '-std=c++11' [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    FRFXLLCreateContext.cpp
    FRFXLLCreateFeatureSet.cpp
    FRFXLLCreateFeatureSet.cpp
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\serializeFpData.h(279,33): warning C4267: 'argument': conversion from 'size_t' to 'unsigned int', possible loss of data [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(246,17): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(279,7): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\serializeFpData.h(279,33): warning C4267: 'argument': conversion from 'size_t' to 'unsigned int', possible loss of data [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(246,17): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(279,7): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(90,31): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(90,31):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(283,15):
            see reference to function template instantiation 'std::complex<int32> FingerJetFxOSE::FpRecEngineImpl::Embedded::reduce<int32>(const std::complex<int32> &,uint8)' being compiled
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(90,31): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\freeman.h(123,55): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\freeman.h(123,55):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(142,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::freeman_phasemap<4,uint8,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t>(size_t,size_t,const uint8 *,ori_t *,Tout *)' being compiled
            with
            [
                ori_t=FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t,
                Tout=uint8
            ]
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(45,44): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(45,44):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(146,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::extract_minutia<256,4,FingerJetFxOSE::FpRecEngineImpl::Embedded::Minutia>(const uint8 *,size_t,size_t,const uint8 *,FingerJetFxOSE::top_n<FingerJetFxOSE::FpRecEngineImpl::Embedded::Minutia> &,const FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::Parameters &)' being compiled
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(288,40): warning C4996: 'std::complex<int16>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(310,54): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(358,26): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(360,43): warning C4996: 'std::complex<int16>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(361,64): warning C4996: 'std::complex<int16>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(90,31):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(283,15):
            see reference to function template instantiation 'std::complex<int32> FingerJetFxOSE::FpRecEngineImpl::Embedded::reduce<int32>(const std::complex<int32> &,uint8)' being compiled
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\freeman.h(123,55): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(71,26): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(71,26):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(283,15):
            see reference to function template instantiation 'std::complex<int32> FingerJetFxOSE::FpRecEngineImpl::Embedded::reduce<int32>(const std::complex<int32> &,uint8)' being compiled
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(90,47):
            see reference to function template instantiation 'std::complex<int32> FingerJetFxOSE::FpRecEngineImpl::Embedded::operator >><int32>(const std::complex<int32> &,uint8)' being compiled
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(67,33): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(67,33):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(133,9):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::orientation_map_and_footprint<256,4>(size_t,size_t,const uint8 *,bool,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t *,uint8 *)' being compiled
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(262,5):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::raw_orimap<256,4>(size_t,size_t,const uint8 *,bool,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t *,uint8 *)' being compiled
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(76,45): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(79,27): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(90,30): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(104,30): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(117,30): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(131,62): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(137,44): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(144,34): warning C4996: 'std::complex<int16>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(144,34):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(275,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::smooth_orimap<64,5,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t(const std::complex<int32> &)>(size_t,size_t,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t *,const uint8 *,F (__cdecl &))' being compiled
            with
            [
                F=FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t (const std::complex<int32> &)
            ]
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(45,44): warning C4996: 'std::complex<int16>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\freeman.h(123,55):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(142,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::freeman_phasemap<4,uint8,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t>(size_t,size_t,const uint8 *,ori_t *,Tout *)' being compiled
            with
            [
                ori_t=FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t,
                Tout=uint8
            ]
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(149,24): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(151,27): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(153,43): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\orimap.h(165,62): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(64,29): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(64,29):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(142,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::freeman_phasemap<4,uint8,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t>(size_t,size_t,const uint8 *,ori_t *,Tout *)' being compiled
            with
            [
                ori_t=FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::ori_t,
                Tout=uint8
            ]
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\freeman.h(123,32):
            see reference to function template instantiation 'std::complex<int8> FingerJetFxOSE::FpRecEngineImpl::Embedded::oct_sign<int32>(const std::complex<int32> &,const T &)' being compiled
            with
            [
                T=int32
            ]
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\complex.h(66,29): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(45,44): warning C4996: 'std::complex<int8>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(45,44):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(146,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::extract_minutia<256,4,FingerJetFxOSE::FpRecEngineImpl::Embedded::Minutia>(const uint8 *,size_t,size_t,const uint8 *,FingerJetFxOSE::top_n<FingerJetFxOSE::FpRecEngineImpl::Embedded::Minutia> &,const FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::Parameters &)' being compiled
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(288,40): warning C4996: 'std::complex<int16>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(91,28): warning C4146: unary minus operator applied to unsigned type, result still unsigned [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(91,28):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(146,7):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::extract_minutia<256,4,FingerJetFxOSE::FpRecEngineImpl::Embedded::Minutia>(const uint8 *,size_t,size_t,const uint8 *,FingerJetFxOSE::top_n<FingerJetFxOSE::FpRecEngineImpl::Embedded::Minutia> &,const FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::Parameters &)' being compiled
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(284,74):
            see reference to class template instantiation 'FingerJetFxOSE::FpRecEngineImpl::Embedded::delay<bool,2300>' being compiled
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\delay.h(88,9):
            while compiling class template member function 'FingerJetFxOSE::FpRecEngineImpl::Embedded::delay<bool,2300>::delay(size_t,bool)'
                D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(284,82):
                see the first reference to 'FingerJetFxOSE::FpRecEngineImpl::Embedded::delay<bool,2300>::delay' in 'FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::extract_minutia'
    
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(310,54): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(209,15): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(209,15):
        the template instantiation context (the oldest one first) is
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\FeatureExtraction.h(134,15):
            see reference to function template instantiation 'bool FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::fft_enhance<5,17>(uint8 *,size_t,size_t,size_t)' being compiled
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\fft_enhance.h(122,14):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::enhance_block<5,17>(int32 *,const FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::envelope<32,17> &,const FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::envelope<32,17> &)' being compiled
            D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(329,5):
            see reference to function template instantiation 'void FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::enhance_array<5,FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::ci32(__cdecl *)(FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::ci32,int32,int32)>(int32 *,F)' being compiled
            with
            [
                F=FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::ci32 (__cdecl *)(FingerJetFxOSE::FpRecEngineImpl::Embedded::FeatureExtractionImpl::FFT::ci32,int32,int32)
            ]
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\extract_minutia.h(358,26): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
    
  D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(159,15): warning C4996: 'std::complex<int32>::complex': warning STL4037: The effect of instantiating the template std::complex for any type other than float, double, or long double is unspecified. You can define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING to suppress this warning. [D:\a\fingerprint\fingerprint\target\release\build\nbis-rs-27437145f5b106f6\out\build\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\FRFXLL_static.vcxproj]
    (compiling source file '../../../../../../../../../../crates/nbis-rs/ext/NFIQ2-2.3.0/fingerjetfxose/FingerJetFXOSE/libFRFXLL/src/lib/FRFXLLCreateFeatureSet.cpp')
        D:\a\fingerprint\fingerprint\crates\nbis-rs\ext\NFIQ2-2.3.0\fingerjetfxose\FingerJetFXOSE\libFRFXLL\src\algorithm\block_fft.h(159,15):
Error: Process completed with exit code 1.
)
