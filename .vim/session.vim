let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/Documents/Projects/Rust/ghterm
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +12 Cargo.toml
badd +1 term
badd +43 term://.//27078:/opt/homebrew/bin/fish
badd +17 src/main.rs
badd +2 src/backend.rs
badd +12 src/backend/pr.rs
badd +49 src/backend/gh.rs
badd +96 term://.//1186:/opt/homebrew/bin/fish
badd +4 term://.//21407:/opt/homebrew/bin/fish
badd +61 src/frontend/screen.rs
badd +52 src/frontend/repo_selection.rs
badd +49 src/app.rs
badd +2 src/app/events.rs
badd +2 src/frontend.rs
badd +36 src/frontend/repo_selection_handler.rs
badd +1250 term://.//3783:/opt/homebrew/bin/fish
badd +7 src/logs.rs
badd +7 term://.//8022:/opt/homebrew/bin/fish
argglobal
%argdel
$argadd Cargo.toml
edit src/frontend/repo_selection.rs
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 133 + 102) / 204)
exe 'vert 2resize ' . ((&columns * 70 + 102) / 204)
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 52 - ((22 * winheight(0) + 21) / 43)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
52
normal! 032|
wincmd w
argglobal
if bufexists("term://.//3783:/opt/homebrew/bin/fish") | buffer term://.//3783:/opt/homebrew/bin/fish | else | edit term://.//3783:/opt/homebrew/bin/fish | endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 2329 - ((42 * winheight(0) + 21) / 43)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
2329
normal! 033|
wincmd w
exe 'vert 1resize ' . ((&columns * 133 + 102) / 204)
exe 'vert 2resize ' . ((&columns * 70 + 102) / 204)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFc
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
