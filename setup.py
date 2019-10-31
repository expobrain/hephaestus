from pathlib import Path

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="hephaestus",
    version="0.1.2",
    rust_extensions=[RustExtension("hephaestus.hephaestus", binding=Binding.PyO3)],
    packages=["hephaestus"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    long_description=Path("README.md").read_text(),
    long_description_content_type="text/markdown",
)
