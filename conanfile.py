from os import path
from conan import ConanFile
from conan.tools.scm import Git
from conan.tools.cmake import CMakeToolchain, CMakeDeps, CMake, cmake_layout
from conan.tools.files import copy, update_conandata

class AoC2022(ConanFile):
    name = "aoc2022"
    verison = "0.1.0"
    settings = "os", "arch", "compiler", "build_type"

    def build_requirements(self):
        self.tool_requires("cmake/3.24.1")
        self.tool_requires("ninja/1.11.1")
    
    def requirements(self):
        # self.requires("boost/1.80.0")
        pass

    def source(self):
        git = Git(self)
        sources = self.conan_data["sources"]
        git.clone(url=sources["url"], target=".")
        git.checkout(commit=sources["commit"])

    def layout(self):
        cmake_layout(self)

    def generate(self):
        # This generates "conan_toolchain.cmake" in self.generators_folder
        tc = CMakeToolchain(self)
        # tc.variables["MYVAR"] = "1"
        # tc.preprocessor_definitions["MYDEFINE"] = "2"
        tc.generate()

        # This generates "foo-config.cmake" and "bar-config.cmake" in self.generators_folder
        deps = CMakeDeps(self)
        deps.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()
        # cmake.test()

    def export(self):
        git = Git(self, self.recipe_folder)
        scm_url, scm_commit = git.get_url_and_commit()
        update_conandata(self, {"sources": {"commit": scm_commit, "url": scm_url}})

    def package(self):
        copy(self, "*.h", self.source_folder, path.join(self.package_folder, "api"), keep_path=False)
        copy(self, "*.lib", self.build_folder, path.join(self.package_folder, "lib"), keep_path=False)
        copy(self, "*.dll", self.build_folder, path.join(self.package_folder, "bin"), keep_path=False)