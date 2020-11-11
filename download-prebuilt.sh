mkdir -p $1
cd $1
curl -s https://api.github.com/repos/EmbarkStudios/ash-molten/releases/tags/MoltenVK-$2 \
| grep "browser_download_url.*zip" \
| cut -d : -f 2,3 \
| tr -d \" \
| xargs -n 1 curl -LO
unzip \*.zip -x '__MACOSX/*'