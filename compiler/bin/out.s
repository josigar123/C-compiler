	.text
	.global _main
_main:
	sub sp, sp, #16
	mov x0, #53
	str x0, [sp,12]
	mov x0, 0
	add sp, sp, 16
	