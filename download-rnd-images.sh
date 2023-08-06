# TEST IMAGES SOURCE:
# https://openexr.com/en/latest/_test_images/index.html#test-images

img_dir=rnd_images
if [ -d $img_dir ]
then
    return
fi
mkdir $img_dir

curl https://raw.githubusercontent.com/AcademySoftwareFoundation/openexr-images/main/TestImages/AllHalfValues.exr -o $img_dir/AllHalfValues.exr
curl https://raw.githubusercontent.com/AcademySoftwareFoundation/openexr-images/main/ScanLines/Desk.exr -o $img_dir/Desk.exr
