from setuptools import setup

from setuptools_rust import Binding, RustExtension

setup(
    name="efesto",
    version="0.1.0",
    rust_extensions=[RustExtension("efesto.efesto", binding=Binding.PyO3)],
    packages=["efesto"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
