set ts=4 sw=4 sts=4 et

inoremap <leader>cc <esc>:!cargo check 2>&1 \| less<cr>
noremap <leader>cc :!cargo check 2>&1 \| less<cr>
inoremap <leader>cr <esc>:!cargo run<cr>
noremap <leader>cr :!cargo run<cr>
