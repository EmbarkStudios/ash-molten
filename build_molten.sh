git clone https://github.com/KhronosGroup/MoltenVK.git
cd MoltenVK
sh fetchDependencies
make macos
cp Package/Release/MoltenVK/macOS/static/libMoltenVK.a ../native
