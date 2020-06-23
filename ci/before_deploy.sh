# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc -features="pyo3" --target $TARGET --release -- -C lto

    case $TRAVIS_OS_NAME in
        linux)
	    cp target/$TARGET/release/libobjset.so $stage/objset.so
            ;;
        osx)
	    cp target/$TARGET/release/libobjset.dylib $stage/objset.so
            ;;
    esac

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
