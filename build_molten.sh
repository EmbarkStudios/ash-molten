git submodule update --init
cd MoltenVK
sh fetchDependencies
make macos
cp Package/Release/MoltenVK/macOS/static/libMoltenVK.a ../external
