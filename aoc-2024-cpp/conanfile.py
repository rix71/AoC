from conan import ConanFile
from conan.tools.cmake import cmake_layout


class ExampleRecipe(ConanFile):
    settings = "os", "compiler", "build_type", "arch"
    generators = "CMakeDeps", "CMakeToolchain"

    def requirements(self):
        self.requires("fmt/11.0.2")
        self.requires("flux/cci.20240115")
        self.requires("gtest/1.14.0")
        self.requires("boost/1.86.0")

    def layout(self):
        cmake_layout(self)
