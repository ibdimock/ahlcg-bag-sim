del /Q ..\ahlcg-bag-simulator-gh-pages\*
rmdir /Q /S ..\ahlcg-bag-simulator-gh-pages\pkg
rmdir /Q /S ..\ahlcg-bag-simulator-gh-pages\css
rmdir /Q /S ..\ahlcg-bag-simulator-gh-pages\svgs

cmd /C wasm-pack build --release --target web

mkdir ..\ahlcg-bag-simulator-gh-pages\pkg\
copy /Y pkg\*.js ..\ahlcg-bag-simulator-gh-pages\pkg\
copy /Y pkg\*.wasm ..\ahlcg-bag-simulator-gh-pages\pkg\

cmd /C sass .\ahlcg-simulator-frontend\scss\custom.scss .\ahlcg-simulator-frontend\css\custom.css
cmd /C css-minify -f .\ahlcg-simulator-frontend\css\custom.css

mkdir ..\ahlcg-bag-simulator-gh-pages\css\
copy /Y .\css-dist\custom.min.css ..\ahlcg-bag-simulator-gh-pages\css\

mkdir ..\ahlcg-bag-simulator-gh-pages\svgs\
copy /Y .\ahlcg-simulator-frontend\svgs\* ..\ahlcg-bag-simulator-gh-pages\svgs\

copy /Y .\ahlcg-simulator-frontend\index.html ..\ahlcg-bag-simulator-gh-pages\
copy /Y .\ahlcg-simulator-frontend\chaos.png ..\ahlcg-bag-simulator-gh-pages\
copy /Y .\ahlcg-simulator-frontend\simulator.js ..\ahlcg-bag-simulator-gh-pages\

