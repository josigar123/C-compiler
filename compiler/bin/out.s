	.text
	.global _main

_main:
	
	sub sp, sp, #16
	mov w0, #5
	str w0, [sp, #12]
	
	mov w0, #23
	str w0, [sp, #8]
	
	mov w0, #34
	str w0, [sp, #4]
	
	mov w0, #9
	str w0, [sp, #0]
	
	mov w0, #10
	str w0, [sp, #12]
	
	mov w0, #0
	add sp, sp, #16
	ret
