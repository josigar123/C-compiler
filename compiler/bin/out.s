	.text
	.global _main

_main:
	mov w0, #1
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #2
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #3
	ldr x1, [sp, 12]
	add x0, x0,x1
	add sp, sp, 16
	
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #2
	ldr x1, [sp, 12]
	 mul x0, x1, x0
	add sp, sp, 16
	ldr x1, [sp, 12]
	add x0, x0,x1
	add sp, sp, 16
	
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #1
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #1
	ldr x1, [sp, 12]
	 mul x0, x1, x0
	add sp, sp, 16
	ldr x1, [sp, 12]
	sub x0, x1, x0
	add sp, sp, 16
	
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #5
	ldr x1, [sp, 12]
	sub x0, x1, x0
	add sp, sp, 16
	
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #3
	ldr x1, [sp, 12]
	add x0, x0,x1
	add sp, sp, 16
	
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #2
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #3
	ldr x1, [sp, 12]
	 mul x0, x1, x0
	add sp, sp, 16
	ldr x1, [sp, 12]
	sub x0, x1, x0
	add sp, sp, 16
	
	ret
