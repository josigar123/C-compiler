	.text
	.global _main

_main:
	
	sub sp, sp, #4
	str w0, [sp, #4]
	mov w0, #5
	
	mov w0, #0
	add sp, sp, #4
	ret
