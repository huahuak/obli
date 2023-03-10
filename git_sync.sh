git submodule foreach --recursive git add . && \
git submodule foreach --recursive git commit -m "$1" && \
git submodule foreach --recursive git push  && \

git add . && \
git commit -m "$1" && \
git push 