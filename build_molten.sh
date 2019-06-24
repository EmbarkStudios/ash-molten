git clone https://github.com/KhronosGroup/MoltenVK.git
cd MoltenVK
sh fetchDependencies
xcodebuild -project "$(XCODE_PROJ)" -scheme "$(XCODE_SCHEME_BASE) (macOS only)"
cp Package/Release/MoltenVK/macOS/static/libMoltenVK.a ../native
