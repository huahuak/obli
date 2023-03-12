git submodule foreach --recursive git add . && \
echo "// ------------------------------------ //"
git submodule foreach --recursive git commit -m "$1" & \
echo "// ------------------------------------ //"
git submodule foreach --recursive git push  && \

echo "// ------------------------------------ //"
git add . && \
echo "// ------------------------------------ //"
git commit -m "$1" && \
echo "// ------------------------------------ //"
git push 
