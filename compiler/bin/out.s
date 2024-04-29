	.text
	.global _main
	.global _qnaz
	.global _faz
	.global _baz
	.global _chaz

_main:
	mov w0, #47
	ret
	mov w0, #55
	ret
	mov w0, #47
	ret

_qnaz:
	mov w0, #12
	ret
	mov w0, #55
	ret
	mov w0, #43
	ret

_faz:
	mov w0, #3
	ret
	mov w0, #12
	ret
	mov w0, #55
	ret
	mov w0, #43
	ret

_baz:
	mov w0, #2
	ret
	mov w0, #12
	ret
	mov w0, #55
	ret
	mov w0, #43
	ret

_chaz:
	mov w0, #43
	ret
