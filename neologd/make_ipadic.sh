VERSION="0.0.7"
curl -L https://github.com/neologd/mecab-ipadic-neologd/archive/refs/tags/v${VERSION}.zip > ./mecab-ipadic-neologd.zip
unzip -o mecab-ipadic-neologd.zip
./mecab-ipadic-neologd-${VERSION}/bin/install-mecab-ipadic-neologd --create_user_dic -p $(pwd)/mecab-ipadic-neologd/tmp -y
IPADIC_VERSION=$(find ./mecab-ipadic-neologd-${VERSION}/build/mecab-ipadic-*-neologd-* -type d | awk -F "-" '{print $6"-"$7}')
NEOLOGD_VERSION=$(find ./mecab-ipadic-neologd-${VERSION}/build/mecab-ipadic-*-neologd-* -type d | awk -F "-" '{print $NF}')

lindera-ipadic-neologd ./mecab-ipadic-neologd-${VERSION}/build/mecab-ipadic-${IPADIC_VERSION}-neologd-${NEOLOGD_VERSION} lindera-ipadic

