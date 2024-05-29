	.text
	.global _main

_main:
	
	sub sp, sp, #16
	mov w0, #10
	str w0, [sp, #12]
	
	mov w0, #20
	str w0, [sp, #8]
	
	mov w0, #300
	str w0, [sp, #4]
	
	mov w0, #600
	str w0, [sp, #0]
	
	sub sp, sp, #16
	mov w0, #1
	str w0, [sp, #12]
	
	mov w0, #5
	str w0, [sp, #8]
	
	mov w0, #10
	str w0, [sp, #4]
	
	mov w0, #43
	str w0, [sp, #0]
	
	sub sp, sp, #16
	mov w0, #0
	str w0, [sp, #12]
	
	mov w0, #12
	str w0, [sp, #8]
	
	mov w0, #1
	str w0, [sp, #4]
	
	mov w0, #1
	str w0, [sp, #0]
	
	sub sp, sp, #16
	mov w0, #0
	str w0, [sp, #12]
	
	mov w0, #10
	str w0, [sp, #8]
	
	mov w0, #123
	str w0, [sp, #4]
	
	mov w0, #1
	str w0, [sp, #0]
	
	sub sp, sp, #16
	mov w0, #1
	str w0, [sp, #12]
	
	mov w0, #213
	str w0, [sp, #8]
	
	mov w0, #1213
	str w0, [sp, #4]
	
	mov w0, #123
	str w0, [sp, #0]
	
	mov w0, #89
	str w0, [sp, #0]
	
	ldr w0, [sp, 52]
	add sp, sp, #80
	ret
