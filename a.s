
global main
section .text:

main:
	push rbp
	mov rbp, rsp
	leave
	ret
section .note.GNU-stack
