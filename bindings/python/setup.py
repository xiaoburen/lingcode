from setuptools import setup, find_packages

setup(
    name='lingcode_rime',
    version='0.1.0',
    description='A Rime-like input method framework supporting Pinyin and Double Pinyin.',
    author='Your Name',
    author_email='your.email@example.com',
    url='https://github.com/yourusername/lingcode-rime-rs',
    packages=find_packages(where='src'),
    package_dir={'': 'src'},
    install_requires=[
        # Add any Python dependencies here
    ],
    classifiers=[
        'Programming Language :: Python :: 3',
        'Operating System :: OS Independent',
    ],
    python_requires='>=3.6',
)