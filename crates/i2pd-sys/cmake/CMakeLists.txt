cmake_minimum_required(VERSION 3.14)

# download CPM.cmake
file(
  DOWNLOAD
  https://github.com/cpm-cmake/CPM.cmake/releases/download/v0.40.8/CPM.cmake
  ${CMAKE_CURRENT_BINARY_DIR}/cmake/CPM.cmake
  EXPECTED_HASH SHA256=78ba32abdf798bc616bab7c73aac32a17bbd7b06ad9e26a6add69de8f3ae4791
)
include(${CMAKE_CURRENT_BINARY_DIR}/cmake/CPM.cmake)


CPMAddPackage(
    NAME zlib
    URL https://github.com/madler/zlib/releases/download/v1.3.1/zlib-1.3.1.tar.gz
    OPTIONS 
        "ZLIB_BUILD_STATIC ON"
        "ZLIB_INSTALL_COMPAT_DLL ON"
)
add_library(ZLIB::ZLIB ALIAS zlibstatic)


# set(OPENSSL_TARGET_VERSION "1.1.1w")
#CPMAddPackage(
#    NAME openssl-cmake
#    GIT_REPOSITORY https://github.com/jimmy-park/openssl-cmake.git
#    GIT_TAG main
#)
#CPMAddPackage(
#    NAME openssl-cmake
#    URL https://github.com/jimmy-park/openssl-cmake/archive/main.tar.gz
#    OPTIONS
#    "OPENSSL_CONFIGURE_OPTIONS option1\\\\;option2"
#)

CPMAddPackage(
    NAME libressl
    URL https://github.com/libressl/portable/releases/download/v4.2.0/libressl-4.2.0.tar.gz
    VERSION 4.2
    OPTIONS
        "LIBRESSL_APPS OFF"
        "LIBRESSL_TESTS OFF"
)

#CPMAddPackage(
#    NAME wolfssl
#    URL https://github.com/wolfSSL/wolfssl/archive/refs/tags/v5.8.2-stable.tar.gz
#    VERSION 5.8.2
#    OPTIONS
#        "WOLFSSL_OPENSSLALL yes"
#        "WOLFSSL_OPENSSLEXTRA yes"
#        "WOLFSSL_EXAMPLES OFF"
#        "WOLFSSL_CRYPT_TESTS OFF"
#        "BUILD_SHARED_LIBS OFF"
#        "WOLFSSL_OPENSSLCOEXIST off"
#        "WOLFSSL_INSTALL ON"
#)


CPMAddPackage(
  NAME Boost
  VERSION 1.86.0 # Versions less than 1.85.0 may need patches for installation targets.
  URL https://github.com/boostorg/boost/releases/download/boost-1.86.0/boost-1.86.0-cmake.tar.xz
  URL_HASH SHA256=2c5ec5edcdff47ff55e27ed9560b0a0b94b07bd07ed9928b476150e16b0efc57
  OPTIONS "BOOST_ENABLE_CMAKE ON" "BOOST_SKIP_INSTALL_RULES OFF" # Set `OFF` for installation
          "BUILD_SHARED_LIBS OFF" "BOOST_INCLUDE_LIBRARIES container\\\;asio\\\;program_options\\\;property_tree\\\;filesystem" # Note the escapes!
          "BOOST_INSTALL_LAYOUT system"
)

# Only build android-ifaddrs on Android
if(CMAKE_SYSTEM_NAME STREQUAL "Android")
    # Fetch android-ifaddrs from GitHub
    CPMAddPackage(
        NAME android-ifaddrs
        GITHUB_REPOSITORY PurpleI2P/android-ifaddrs
        GIT_TAG f302cc7  # latest commit as of 2024
    )

    # Create a static library from the fetched sources
    add_library(android-ifaddrs STATIC
        ${android-ifaddrs_SOURCE_DIR}/ifaddrs.cpp
        ${android-ifaddrs_SOURCE_DIR}/bionic_netlink.cpp
    )

    # Expose headers to consumers
    target_include_directories(android-ifaddrs
        PUBLIC
            $<BUILD_INTERFACE:${android-ifaddrs_SOURCE_DIR}>
    )

    # Ensure position-independent code (required for static libs used in shared libs)
    set_target_properties(android-ifaddrs PROPERTIES
        POSITION_INDEPENDENT_CODE ON
        CXX_STANDARD 17
        CXX_STANDARD_REQUIRED ON
    )
endif()

CPMAddPackage(
    NAME i2pd
    URL "https://github.com/PurpleI2P/i2pd/archive/refs/tags/2.58.0.tar.gz"
    DOWNLOAD_ONLY YES
    VERSION 2.58.0
)

if(i2pd_ADDED)
    add_subdirectory(external/i2pd)
endif()
