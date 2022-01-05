del /Q ..\ahlcg-bag-sim-gh-pages\*
rmdir /Q /S ..\ahlcg-bag-sim-gh-pages\pkg
rmdir /Q /S ..\ahlcg-bag-simu-gh-pages\css
rmdir /Q /S ..\ahlcg-bag-sim-gh-pages\svgs

cmd /C wasm-pack build --release --target web

mkdir ..\ahlcg-bag-sim-gh-pages\pkg\
copy /Y pkg\*.js ..\ahlcg-bag-sim-gh-pages\pkg\
copy /Y pkg\*.wasm ..\ahlcg-bag-sim-gh-pages\pkg\

cmd /C npx sass --no-source-map .\ahlcg-sim-frontend\scss\custom.scss .\ahlcg-sim-frontend\css\custom.css

mkdir ..\ahlcg-bag-sim-gh-pages\css\
copy /Y .\ahlcg-sim-frontend\css\custom.css ..\ahlcg-bag-sim-gh-pages\css\

mkdir ..\ahlcg-bag-sim-gh-pages\svgs\
copy /Y .\ahlcg-sim-frontend\svgs\* ..\ahlcg-bag-sim-gh-pages\svgs\

copy /Y .\ahlcg-sim-frontend\index.html ..\ahlcg-bag-sim-gh-pages\
copy /Y .\ahlcg-sim-frontend\chaos.png ..\ahlcg-bag-sim-gh-pages\
copy /Y .\ahlcg-sim-frontend\simulator.js ..\ahlcg-bag-sim-gh-pages\
