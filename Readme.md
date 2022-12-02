# AoC 2022

## C++
### Setup
```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

```
conan profile new --detect default
conan profile update settings.compiler.libcxx=libstdc++11 default
cp ~/.conan/profiles/default ~/.conan/profiles/release
cp ~/.conan/profiles/default ~/.conan/profiles/debug
sed -i -e "s/build_type=Release/build_type=Debug/" ~/.conan/profiles/debug
```

### Build
```
conan install . -pr:b=debug -pr:h=debug -if build
conan build . -if build
```

## Rust
### Setup
### Build
