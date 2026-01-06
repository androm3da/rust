use rustc_abi::TyAbiInterface;

use crate::callconv::{ArgAbi, FnAbi};

fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.is_ignore() {
        // Already correctly marked as ignore (e.g., zero-sized types)
        return;
    }

    if !ret.layout.is_sized() {
        return;
    }
    
    let size = ret.layout.size;

    if ret.layout.is_aggregate() {
        if size.bits() == 0 {
            // Zero-sized types - should be ignored in calling convention
            // This shouldn't happen since they should already be marked as Ignore above
            return;
        } else if size.bits() > 64 {
            // Large aggregates (> 64 bits) are returned indirectly (by hidden pointer parameter)
            ret.make_indirect();
        } else if size.bits() > 32 {
            // Medium aggregates (33-64 bits) are returned in register pair R1:R0
            // Cast to i64 to represent the register pair
            ret.cast_to(rustc_abi::Reg::i64());
        } else {
            // Small aggregates (≤ 32 bits) are returned in R0, preserving layout
            // Cast to appropriate integer type based on actual size
            if size.bits() <= 8 {
                ret.cast_to(rustc_abi::Reg::i8());
            } else if size.bits() <= 16 {
                ret.cast_to(rustc_abi::Reg::i16());
            } else {
                ret.cast_to(rustc_abi::Reg::i32());
            }
        }
    } else {
        // Non-aggregates (scalars): let them be returned as-is
        // Hexagon ABI doesn't require extending non-aggregate return values
        return;
    }
}

fn classify_arg<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.is_ignore() {
        // Already correctly marked as ignore (e.g., zero-sized types)
        return;
    }

    if arg.layout.pass_indirectly_in_non_rustic_abis(cx) {
        arg.make_indirect();
        return;
    }

    if !arg.layout.is_sized() {
        // Unsized types are not supported in the hexagon ABI - skip them
        return;
    }

    let size = arg.layout.size;

    if arg.layout.is_aggregate() {
        if size.bits() == 0 {
            // Zero-sized types (like Empty struct) - should be ignored in calling convention
            // This shouldn't happen since they should already be marked as Ignore above
            return;
        } else if size.bits() > 64 {
            // Large aggregates (> 64 bits) are passed on stack by value
            // According to Hexagon ABI: "Candidates larger than 64 bits are passed on the stack"
            // Section 3.2.2: "Data allocated on the stack is only guaranteed to be up to 8-byte aligned"
            // Use byval (pass by value on stack) with 8-byte alignment as per Hexagon ABI
            arg.pass_by_stack_offset(Some(rustc_abi::Align::from_bytes(8).unwrap()));
        } else if size.bits() > 32 {
            // Medium aggregates (33-64 bits) are passed in register pair
            // Cast to i64 to represent the register pair
            arg.cast_to(rustc_abi::Reg::i64());
        } else {
            // Small aggregates (≤ 32 bits) are passed in single register, preserving layout
            // Cast to appropriate integer type based on actual size
            if size.bits() <= 8 {
                arg.cast_to(rustc_abi::Reg::i8());
            } else if size.bits() <= 16 {
                arg.cast_to(rustc_abi::Reg::i16());
            } else {
                arg.cast_to(rustc_abi::Reg::i32());
            }
        }
    } else {
        // Non-aggregates (scalars): let them be passed as-is
        // Hexagon ABI doesn't require extending non-aggregate types
        return;
    }
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(cx, arg);
    }
}
