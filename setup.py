from setuptools import setup, find_packages

def build_native(spec):
    # Step 1: build the rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='.'
    )

    # Step 2: add a cffi module based on the dylib we built
    #
    # We use lambdas here for dylib and header_filename so that those are
    # only called after the external build finished.
    spec.add_cffi_module(
        module_path='minecraft_nether_gen_rs._native',
        dylib=lambda: build.find_dylib('minecraft_nether_gen_rs', in_path='target/release'),
        header_filename=lambda: build.find_header('minecraft_nether_gen_rs.h', in_path='target'),
        rtld_flags=['NOW', 'NODELETE']
    )


setup(
    name='minecraft_nether_gen_rs',
    version='1.1.1',
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake'],
    install_requires=['milksnake'],
    milksnake_tasks=[
        build_native,
    ]
)