#! /bin/sh -e
# This script demonstrates archive and create action on frameworks and libraries
#
# @author Boris Bielik

# Release dir path
OUTPUT_DIR_PATH=$1

if [[ -z $1 ]]; then
    echo "Output dir was not set. try to run ./scripts/create_xcframeworks.sh Products"
    exit 1;
fi

function archivePathSimulator {
  local DIR=${OUTPUT_DIR_PATH}/archives/"${1}-SIMULATOR"
  echo "${DIR}"
}

function archivePathDevice {
  local DIR=${OUTPUT_DIR_PATH}/archives/"${1}-DEVICE"
  echo "${DIR}"
}

# Archive takes 3 params
#
# 1st == SCHEME
# 2nd == destination
# 3rd == archivePath
function archive {
    echo "▸ Starts archiving the scheme: ${1} for destination: ${2};\n▸ Archive path: ${3}.xcarchive"
    xcodebuild archive \
    -workspace XCFrameworks.xcworkspace \
    -scheme ${1} \
    -destination "${2}" \
    -archivePath "${3}" \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES
}

## Builds archive for iOS simulator & device
#function buildArchive {
#  SCHEME=${1}
#
#  archive $SCHEME "generic/platform=iOS Simulator" $(archivePathSimulator $SCHEME)
#  archive $SCHEME "generic/platform=iOS" $(archivePathDevice $SCHEME)
#}

## Creates xc framework
#function createXCFramework {
#  FRAMEWORK_ARCHIVE_PATH_POSTFIX=".xcarchive/Products/Library/Frameworks"
#  FRAMEWORK_SIMULATOR_DIR="$(archivePathSimulator $1)${FRAMEWORK_ARCHIVE_PATH_POSTFIX}"
#  FRAMEWORK_DEVICE_DIR="$(archivePathDevice $1)${FRAMEWORK_ARCHIVE_PATH_POSTFIX}"
#
#  xcodebuild -create-xcframework \
#            -framework ${FRAMEWORK_SIMULATOR_DIR}/${SIMULATOR_DEVICE_FRAMEWORK}.framework \
#            -framework ${FRAMEWORK_DEVICE_DIR}/${IOS_DEVICE_FRAMEWORK}.framework \
#            -output ${OUTPUT_DIR_PATH}/xcframeworks/${1}.xcframework
#}

### Static Libraries cant be turned into frameworks
function createXCFrameworkForStaticLibrary {

#  LIBRARY_ARCHIVE_PATH_POSTFIX=".xcarchive/Products/usr/local/lib"
#  LIBRARY_SIMULATOR_DIR="$(archivePathSimulator "$1-Simulator")${LIBRARY_ARCHIVE_PATH_POSTFIX}"
#  LIBRARY_DEVICE_DIR="$(archivePathDevice "$1-iOS")${LIBRARY_ARCHIVE_PATH_POSTFIX}"

  xcodebuild -create-xcframework \
            -library ../target/x86_64-apple-ios/release/libautomerge.a \
            -library ../target/aarch64-apple-ios/release/libautomerge.a \
            -headers ../automerge-c/automerge.h \
            -output ${OUTPUT_DIR_PATH}/xcframeworks/${1}.xcframework
}

echo "#####################"
echo "▸ Cleaning the dir: ${OUTPUT_DIR_PATH}"
rm -rf $OUTPUT_DIR_PATH

#### Static Library ####
LIBRARY=AutomergeRSBackend

#echo "▸ Archive $LIBRARY"
#archive "$LIBRARY" "generic/platform=iOS Simulator" $(archivePathSimulator $LIBRARY)
#archive "$LIBRARY" "generic/platform=iOS" $(archivePathDevice $LIBRARY)

echo "▸ Create $FRAMEWORK.xcframework"
createXCFrameworkForStaticLibrary ${LIBRARY}

#### Dynamic Framework ####

#DYNAMIC_FRAMEWORK=AutomergeRSBackend
#
#echo "▸ Archive $DYNAMIC_FRAMEWORK"
#archive ${SIMULATOR_DEVICE_FRAMEWORK} "generic/platform=iOS Simulator" $(archivePathSimulator $DYNAMIC_FRAMEWORK)
#archive ${IOS_DEVICE_FRAMEWORK} "generic/platform=iOS" $(archivePathDevice $DYNAMIC_FRAMEWORK)
#
#echo "▸ Create $DYNAMIC_FRAMEWORK.xcframework"
#createXCFramework ${DYNAMIC_FRAMEWORK}