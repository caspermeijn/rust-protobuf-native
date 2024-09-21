// Copyright Materialize, Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let build_conformance = true;

    let out_dir =
        &PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set"));

    let version = "27.1";

    let build_dir = &out_dir.join(format!("build-protobuf-{}", version));

    let install_dir = cmake::Config::new("protobuf")
        .define("ABSL_PROPAGATE_CXX_STD", "ON")
        .define("protobuf_BUILD_TESTS", "OFF")
        .define(
            "protobuf_BUILD_CONFORMANCE",
            if build_conformance { "ON" } else { "OFF" },
        )
        .define("protobuf_DEBUG_POSTFIX", "")
        .define("CMAKE_CXX_STANDARD", "17")
        // CMAKE_INSTALL_LIBDIR is inferred as "lib64" on some platforms, but we
        // want a stable location that we can add to the linker search path.
        // Since we're not actually installing to /usr or /usr/local, there's no
        // harm to always using "lib" here.
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .out_dir(build_dir)
        .build();

    if build_conformance {
        // Install the conformance-test-runner binary, since it isn't done automatically.
        std::fs::copy(
            build_dir.join("build").join("conformance_test_runner"),
            install_dir.join("bin").join("conformance-test-runner"),
        )?;

        std::fs::copy(
            "protobuf/conformance/conformance.proto",
            install_dir.join("include").join("conformance.proto"),
        )?;
    }

    println!("cargo:rustc-env=INSTALL_DIR={}", install_dir.display());
    println!("cargo:CXXBRIDGE_DIR0={}/include", install_dir.display());
    Ok(())
}
