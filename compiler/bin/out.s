	.text
	.global _main

_main:
	
                        
                        
                        
                        
                        
                        
	mov w0, #1
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #2
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	blt .L2
                        
	mov w0, 0
                        
	 b .L3
                        
.L2:
                        
	mov w0, 1
                        
.L3:
                        
	add sp, sp, 16
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #3
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	blt .L4
                        
	mov w0, 0
                        
	 b .L5
                        
.L4:
                        
	mov w0, 1
                        
.L5:
                        
	add sp, sp, 16
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #4
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	blt .L6
                        
	mov w0, 0
                        
	 b .L7
                        
.L6:
                        
	mov w0, 1
                        
.L7:
                        
	add sp, sp, 16
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #5
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	bgt .L8
                        
	mov w0, 0
                        
	 b .L9
                        
.L8:
                        
	mov w0, 1
                        
.L9:
                        
	add sp, sp, 16
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #6
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	ble .L10
                        
	mov x0, 0
                        
	b   .L11
                        
.L10:
                        
	mov x0, 1
                        
.L11:
                        
	add sp, sp, 16
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #0
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	bge .L12
                        
	mov x0, 0
                        
	b   .L13
                        
.L12:
                        
	mov x0, 1
                        
.L13:
                        
	add sp, sp, 16
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
                        
                        
                        
	mov w0, #1
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #2
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	ble .L14
                        
	mov x0, 0
                        
	b   .L15
                        
.L14:
                        
	mov x0, 1
                        
.L15:
                        
	add sp, sp, 16
                        
	sub sp, sp, #16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #1
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	cset x0, eq
                        
	add sp, sp, 16
                        
	sub sp, sp, #16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #33
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #1
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #2
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	
	mov w0, #23
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #2
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	mul x0, x1, x0
                            
	add sp, sp, 16
	sub sp, sp, #16
	str x0, [sp, 12]
	
	mov w0, #5
	ldr x1, [sp, 12]
	sdiv x0, x1, x0
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
                        
	ldr x1, [sp, 12]
                        
	cmp x0, x1
                        
	cset x0, ne
                        
	add sp, sp , 16
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	beq .L16
                            
	cmp x0, 0
                            
	beq .L16
                            
	mov x0, 1
                            
	b   .L17
                            
.L16:
                            
	mov w0, 0
                            
.L17:
                            
	add sp, sp, 16
	ret
