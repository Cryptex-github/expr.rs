from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="expr_rs",
    version="0.0.0",
    rust_extensions=[RustExtension("expr_rs.expr_rs", binding=Binding.PyO3, features=["python"])],
    packages=["expr_rs"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
