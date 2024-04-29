	.text
	.global _main
	.global _qnaz
	.global _faz
	.global _baz
	.global _chaz

_main:
	mov w0, #1
	ret

_qnaz:
	mov w0, #12
	ret

_faz:
	mov w0, #3
	ret

_baz:
	mov w0, #2
	ret

_chaz:
	mov w0, #43
	ret
