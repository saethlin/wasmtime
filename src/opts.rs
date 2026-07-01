pub fn constructor_simplify<C: Context>(
    ctx: &mut C,
    arg0: Value,
    returns: &mut (impl Extend<Value> + Length),
) -> () {
    let mut v1 = C::inst_data_value_etor_returns::default();
    C::inst_data_value_etor(ctx, arg0, &mut v1);
    let mut v1 = v1.into_context_iter();
    while let Some(v2) = v1.next(ctx) {
        match &v2.1 {
            &InstructionData::Binary {
                opcode: ref v5,
                args: ref v6,
            } => {
                if (|| -> bool {
                    match v5 {
                        &Opcode::Bor => {
                            if (|| -> bool {
                                let v7 = C::unpack_value_array_2(ctx, v6);
                                let mut v10 = C::inst_data_value_etor_returns::default();
                                C::inst_data_value_etor(ctx, v7.1, &mut v10);
                                let mut v10 = v10.into_context_iter();
                                while let Some(v11) = v10.next(ctx) {
                                    match &v11.1 {
                                        &InstructionData::Binary {
                                            opcode: ref v138,
                                            args: ref v139,
                                        } => {
                                            if (|| -> bool {
                                                match v138 {
                                                    &Opcode::Band => {
                                                        if (|| -> bool {
                                                            if v2.0 == v11.0 {
                                                                let mut v18 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                let mut v18 = v18.into_context_iter();
                                                                while let Some(v19) = v18.next(ctx) {
                                                                    match &v19.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v102,
                                                                            args: ref v103,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v102 {
                                                                                    &Opcode::Band => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                let mut v107 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                                                let mut v107 = v107.into_context_iter();
                                                                                                while let Some(v108) = v107.next(ctx) {
                                                                                                    if let &InstructionData::Unary {
                                                                                                        opcode: ref v111,
                                                                                                        arg: v112,
                                                                                                    } = &v108.1 {
                                                                                                        if let &Opcode::Bnot = v111 {
                                                                                                            if v2.0 == v108.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                if v112 == v140.0 {
                                                                                                                    let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                                    let mut v263 = v263.into_context_iter();
                                                                                                                    while let Some(v264) = v263.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v632,
                                                                                                                            arg: v633,
                                                                                                                        } = &v264.1 {
                                                                                                                            if let &Opcode::Bnot = v632 {
                                                                                                                                if v2.0 == v264.0 {
                                                                                                                                    if v104.1 == v633 {
                                                                                                                                        let v879 = constructor_bxor(ctx, v2.0, v112, v104.1);
                                                                                                                                        // Rule at src/opts/bitops.isle line 549.
                                                                                                                                        returns.extend(Some(v879));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                        let v875 = constructor_bxor(ctx, v2.0, v104.1, v112);
                                                                                                                                        // Rule at src/opts/bitops.isle line 554.
                                                                                                                                        returns.extend(Some(v875));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    if v104.1 == v140.1 {
                                                                                                                        // Rule at src/opts/bitops.isle line 471.
                                                                                                                        returns.extend(Some(v104.1));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v112 == v140.1 {
                                                                                                                    let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                                    let mut v279 = v279.into_context_iter();
                                                                                                                    while let Some(v280) = v279.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v635,
                                                                                                                            arg: v636,
                                                                                                                        } = &v280.1 {
                                                                                                                            if let &Opcode::Bnot = v635 {
                                                                                                                                if v2.0 == v280.0 {
                                                                                                                                    if v104.1 == v636 {
                                                                                                                                        let v875 = constructor_bxor(ctx, v2.0, v104.1, v112);
                                                                                                                                        // Rule at src/opts/bitops.isle line 552.
                                                                                                                                        returns.extend(Some(v875));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                        let v879 = constructor_bxor(ctx, v2.0, v112, v104.1);
                                                                                                                                        // Rule at src/opts/bitops.isle line 553.
                                                                                                                                        returns.extend(Some(v879));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    if v104.1 == v140.0 {
                                                                                                                        // Rule at src/opts/bitops.isle line 467.
                                                                                                                        returns.extend(Some(v104.1));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                let mut v114 = v114.into_context_iter();
                                                                                                while let Some(v115) = v114.next(ctx) {
                                                                                                    if let &InstructionData::Unary {
                                                                                                        opcode: ref v118,
                                                                                                        arg: v119,
                                                                                                    } = &v115.1 {
                                                                                                        if let &Opcode::Bnot = v118 {
                                                                                                            if v2.0 == v115.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                if v119 == v140.0 {
                                                                                                                    let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                                    let mut v263 = v263.into_context_iter();
                                                                                                                    while let Some(v264) = v263.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v632,
                                                                                                                            arg: v633,
                                                                                                                        } = &v264.1 {
                                                                                                                            if let &Opcode::Bnot = v632 {
                                                                                                                                if v104.0 == v633 {
                                                                                                                                    if v2.0 == v264.0 {
                                                                                                                                        let v873 = constructor_bxor(ctx, v2.0, v104.0, v119);
                                                                                                                                        // Rule at src/opts/bitops.isle line 550.
                                                                                                                                        returns.extend(Some(v873));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                        let v880 = constructor_bxor(ctx, v2.0, v119, v104.0);
                                                                                                                                        // Rule at src/opts/bitops.isle line 551.
                                                                                                                                        returns.extend(Some(v880));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    if v104.0 == v140.1 {
                                                                                                                        // Rule at src/opts/bitops.isle line 469.
                                                                                                                        returns.extend(Some(v104.0));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v119 == v140.1 {
                                                                                                                    let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                                    let mut v279 = v279.into_context_iter();
                                                                                                                    while let Some(v280) = v279.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v635,
                                                                                                                            arg: v636,
                                                                                                                        } = &v280.1 {
                                                                                                                            if let &Opcode::Bnot = v635 {
                                                                                                                                if v104.0 == v636 {
                                                                                                                                    if v2.0 == v280.0 {
                                                                                                                                        let v873 = constructor_bxor(ctx, v2.0, v104.0, v119);
                                                                                                                                        // Rule at src/opts/bitops.isle line 548.
                                                                                                                                        returns.extend(Some(v873));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                        let v880 = constructor_bxor(ctx, v2.0, v119, v104.0);
                                                                                                                                        // Rule at src/opts/bitops.isle line 555.
                                                                                                                                        returns.extend(Some(v880));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    if v104.0 == v140.0 {
                                                                                                                        // Rule at src/opts/bitops.isle line 465.
                                                                                                                        returns.extend(Some(v104.0));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                if v104.0 == v140.0 {
                                                                                                    let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                    C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                    let mut v263 = v263.into_context_iter();
                                                                                                    while let Some(v264) = v263.next(ctx) {
                                                                                                        if let &InstructionData::Unary {
                                                                                                            opcode: ref v632,
                                                                                                            arg: v633,
                                                                                                        } = &v264.1 {
                                                                                                            if let &Opcode::Bnot = v632 {
                                                                                                                if v104.1 == v633 {
                                                                                                                    if v2.0 == v264.0 {
                                                                                                                        // Rule at src/opts/bitops.isle line 464.
                                                                                                                        returns.extend(Some(v104.0));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    let v701 = constructor_bor(ctx, v2.0, v104.1, v140.1);
                                                                                                    let v702 = constructor_band(ctx, v2.0, v701, v104.0);
                                                                                                    // Rule at src/opts/bitops.isle line 271.
                                                                                                    returns.extend(Some(v702));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v703 = constructor_bor(ctx, v2.0, v140.1, v104.1);
                                                                                                    let v704 = constructor_band(ctx, v2.0, v703, v104.0);
                                                                                                    // Rule at src/opts/bitops.isle line 272.
                                                                                                    returns.extend(Some(v704));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v104.1 == v140.0 {
                                                                                                    let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                    C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                    let mut v263 = v263.into_context_iter();
                                                                                                    while let Some(v264) = v263.next(ctx) {
                                                                                                        if let &InstructionData::Unary {
                                                                                                            opcode: ref v632,
                                                                                                            arg: v633,
                                                                                                        } = &v264.1 {
                                                                                                            if let &Opcode::Bnot = v632 {
                                                                                                                if v104.0 == v633 {
                                                                                                                    if v2.0 == v264.0 {
                                                                                                                        // Rule at src/opts/bitops.isle line 468.
                                                                                                                        returns.extend(Some(v104.1));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    let v693 = constructor_bor(ctx, v2.0, v104.0, v140.1);
                                                                                                    let v694 = constructor_band(ctx, v2.0, v693, v104.1);
                                                                                                    // Rule at src/opts/bitops.isle line 267.
                                                                                                    returns.extend(Some(v694));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v699 = constructor_bor(ctx, v2.0, v140.1, v104.0);
                                                                                                    let v700 = constructor_band(ctx, v2.0, v699, v104.1);
                                                                                                    // Rule at src/opts/bitops.isle line 270.
                                                                                                    returns.extend(Some(v700));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v104.0 == v140.1 {
                                                                                                    let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                    C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                    let mut v279 = v279.into_context_iter();
                                                                                                    while let Some(v280) = v279.next(ctx) {
                                                                                                        if let &InstructionData::Unary {
                                                                                                            opcode: ref v635,
                                                                                                            arg: v636,
                                                                                                        } = &v280.1 {
                                                                                                            if let &Opcode::Bnot = v635 {
                                                                                                                if v104.1 == v636 {
                                                                                                                    if v2.0 == v280.0 {
                                                                                                                        // Rule at src/opts/bitops.isle line 466.
                                                                                                                        returns.extend(Some(v104.0));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    let v695 = constructor_bor(ctx, v2.0, v140.0, v104.1);
                                                                                                    let v696 = constructor_band(ctx, v2.0, v695, v104.0);
                                                                                                    // Rule at src/opts/bitops.isle line 268.
                                                                                                    returns.extend(Some(v696));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v697 = constructor_bor(ctx, v2.0, v104.1, v140.0);
                                                                                                    let v698 = constructor_band(ctx, v2.0, v697, v104.0);
                                                                                                    // Rule at src/opts/bitops.isle line 269.
                                                                                                    returns.extend(Some(v698));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v104.1 == v140.1 {
                                                                                                    let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                    C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                    let mut v279 = v279.into_context_iter();
                                                                                                    while let Some(v280) = v279.next(ctx) {
                                                                                                        if let &InstructionData::Unary {
                                                                                                            opcode: ref v635,
                                                                                                            arg: v636,
                                                                                                        } = &v280.1 {
                                                                                                            if let &Opcode::Bnot = v635 {
                                                                                                                if v104.0 == v636 {
                                                                                                                    if v2.0 == v280.0 {
                                                                                                                        // Rule at src/opts/bitops.isle line 470.
                                                                                                                        returns.extend(Some(v104.1));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    let v689 = constructor_bor(ctx, v2.0, v104.0, v140.0);
                                                                                                    let v690 = constructor_band(ctx, v2.0, v689, v104.1);
                                                                                                    // Rule at src/opts/bitops.isle line 265.
                                                                                                    returns.extend(Some(v690));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v691 = constructor_bor(ctx, v2.0, v140.0, v104.0);
                                                                                                    let v692 = constructor_band(ctx, v2.0, v691, v104.1);
                                                                                                    // Rule at src/opts/bitops.isle line 266.
                                                                                                    returns.extend(Some(v692));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_131
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                let mut v263 = v263.into_context_iter();
                                                                                                while let Some(v264) = v263.next(ctx) {
                                                                                                    if let &InstructionData::Unary {
                                                                                                        opcode: ref v632,
                                                                                                        arg: v633,
                                                                                                    } = &v264.1 {
                                                                                                        if let &Opcode::Bnot = v632 {
                                                                                                            if v2.0 == v264.0 {
                                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                                if v104.0 == v140.0 {
                                                                                                                    if v104.1 == v633 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 559.
                                                                                                                        returns.extend(Some(v243));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v104.0 == v633 {
                                                                                                                    if v104.1 == v140.0 {
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 561.
                                                                                                                        returns.extend(Some(v244));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                let mut v279 = v279.into_context_iter();
                                                                                                while let Some(v280) = v279.next(ctx) {
                                                                                                    if let &InstructionData::Unary {
                                                                                                        opcode: ref v635,
                                                                                                        arg: v636,
                                                                                                    } = &v280.1 {
                                                                                                        if let &Opcode::Bnot = v635 {
                                                                                                            if v2.0 == v280.0 {
                                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                                if v104.0 == v140.1 {
                                                                                                                    if v104.1 == v636 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 563.
                                                                                                                        returns.extend(Some(v243));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v104.0 == v636 {
                                                                                                                    if v104.1 == v140.1 {
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 565.
                                                                                                                        returns.extend(Some(v244));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_132
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_130
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v29,
                                                                            arg: v30,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                if let &Opcode::Bnot = v29 {
                                                                                    if v2.0 == v19.0 {
                                                                                        let mut v654 = C::inst_data_value_etor_returns::default();
                                                                                        C::inst_data_value_etor(ctx, v30, &mut v654);
                                                                                        let mut v654 = v654.into_context_iter();
                                                                                        while let Some(v655) = v654.next(ctx) {
                                                                                            if let &InstructionData::Binary {
                                                                                                opcode: ref v658,
                                                                                                args: ref v659,
                                                                                            } = &v655.1 {
                                                                                                match v658 {
                                                                                                    &Opcode::Bor => {
                                                                                                        if (|| -> bool {
                                                                                                            if v2.0 == v655.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                let v660 = C::unpack_value_array_2(ctx, v659);
                                                                                                                if v140.0 == v660.0 {
                                                                                                                    if v140.1 == v660.1 {
                                                                                                                        let v663 = constructor_bxor(ctx, v2.0, v660.0, v660.1);
                                                                                                                        let v664 = constructor_bnot(ctx, v2.0, v663);
                                                                                                                        // Rule at src/opts/bitops.isle line 672.
                                                                                                                        returns.extend(Some(v664));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v665 = constructor_bxor(ctx, v2.0, v660.1, v660.0);
                                                                                                                        let v666 = constructor_bnot(ctx, v2.0, v665);
                                                                                                                        // Rule at src/opts/bitops.isle line 678.
                                                                                                                        returns.extend(Some(v666));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                    let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                                    let mut v263 = v263.into_context_iter();
                                                                                                                    while let Some(v264) = v263.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v632,
                                                                                                                            arg: v633,
                                                                                                                        } = &v264.1 {
                                                                                                                            if let &Opcode::Bnot = v632 {
                                                                                                                                if v633 == v660.1 {
                                                                                                                                    if v2.0 == v264.0 {
                                                                                                                                        let v743 = constructor_bnot(ctx, v2.0, v660.1);
                                                                                                                                        // Rule at src/opts/bitops.isle line 658.
                                                                                                                                        returns.extend(Some(v743));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v140.0 == v660.1 {
                                                                                                                    if v140.1 == v660.0 {
                                                                                                                        let v665 = constructor_bxor(ctx, v2.0, v660.1, v660.0);
                                                                                                                        let v666 = constructor_bnot(ctx, v2.0, v665);
                                                                                                                        // Rule at src/opts/bitops.isle line 674.
                                                                                                                        returns.extend(Some(v666));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v663 = constructor_bxor(ctx, v2.0, v660.0, v660.1);
                                                                                                                        let v664 = constructor_bnot(ctx, v2.0, v663);
                                                                                                                        // Rule at src/opts/bitops.isle line 676.
                                                                                                                        returns.extend(Some(v664));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                    let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                                    let mut v263 = v263.into_context_iter();
                                                                                                                    while let Some(v264) = v263.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v632,
                                                                                                                            arg: v633,
                                                                                                                        } = &v264.1 {
                                                                                                                            if let &Opcode::Bnot = v632 {
                                                                                                                                if v633 == v660.0 {
                                                                                                                                    if v2.0 == v264.0 {
                                                                                                                                        let v739 = constructor_bnot(ctx, v2.0, v660.0);
                                                                                                                                        // Rule at src/opts/bitops.isle line 656.
                                                                                                                                        returns.extend(Some(v739));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                                C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                                let mut v279 = v279.into_context_iter();
                                                                                                                while let Some(v280) = v279.next(ctx) {
                                                                                                                    if let &InstructionData::Unary {
                                                                                                                        opcode: ref v635,
                                                                                                                        arg: v636,
                                                                                                                    } = &v280.1 {
                                                                                                                        if let &Opcode::Bnot = v635 {
                                                                                                                            if v2.0 == v280.0 {
                                                                                                                                if v636 == v660.0 {
                                                                                                                                    if v140.1 == v660.1 {
                                                                                                                                        let v739 = constructor_bnot(ctx, v2.0, v660.0);
                                                                                                                                        // Rule at src/opts/bitops.isle line 652.
                                                                                                                                        returns.extend(Some(v739));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                if v636 == v660.1 {
                                                                                                                                    if v140.1 == v660.0 {
                                                                                                                                        let v743 = constructor_bnot(ctx, v2.0, v660.1);
                                                                                                                                        // Rule at src/opts/bitops.isle line 654.
                                                                                                                                        returns.extend(Some(v743));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            false
                                                                                                        }
                                                                                                        )() { return true; } // __isle_arm_134
                                                                                                    }
                                                                                                    &Opcode::Bxor => {
                                                                                                        if (|| -> bool {
                                                                                                            if v2.0 == v655.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                let v660 = C::unpack_value_array_2(ctx, v659);
                                                                                                                if v140.0 == v660.0 {
                                                                                                                    if v140.1 == v660.1 {
                                                                                                                        let v663 = constructor_bxor(ctx, v2.0, v660.0, v660.1);
                                                                                                                        let v664 = constructor_bnot(ctx, v2.0, v663);
                                                                                                                        // Rule at src/opts/bitops.isle line 230.
                                                                                                                        returns.extend(Some(v664));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v665 = constructor_bxor(ctx, v2.0, v660.1, v660.0);
                                                                                                                        let v666 = constructor_bnot(ctx, v2.0, v665);
                                                                                                                        // Rule at src/opts/bitops.isle line 236.
                                                                                                                        returns.extend(Some(v666));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v140.0 == v660.1 {
                                                                                                                    if v140.1 == v660.0 {
                                                                                                                        let v665 = constructor_bxor(ctx, v2.0, v660.1, v660.0);
                                                                                                                        let v666 = constructor_bnot(ctx, v2.0, v665);
                                                                                                                        // Rule at src/opts/bitops.isle line 232.
                                                                                                                        returns.extend(Some(v666));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v663 = constructor_bxor(ctx, v2.0, v660.0, v660.1);
                                                                                                                        let v664 = constructor_bnot(ctx, v2.0, v663);
                                                                                                                        // Rule at src/opts/bitops.isle line 234.
                                                                                                                        returns.extend(Some(v664));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            false
                                                                                                        }
                                                                                                        )() { return true; } // __isle_arm_135
                                                                                                    }
                                                                                                    _ => {}
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                        if v30 == v140.0 {
                                                                                            let v640 = constructor_bnot(ctx, v2.0, v30);
                                                                                            let v641 = constructor_bor(ctx, v2.0, v140.1, v640);
                                                                                            // Rule at src/opts/bitops.isle line 221.
                                                                                            returns.extend(Some(v641));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                        if v30 == v140.1 {
                                                                                            let v355 = constructor_bor(ctx, v2.0, v140.0, v7.0);
                                                                                            // Rule at src/opts/bitops.isle line 53.
                                                                                            returns.extend(Some(v355));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_133
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                let mut v263 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                let mut v263 = v263.into_context_iter();
                                                                while let Some(v264) = v263.next(ctx) {
                                                                    match &v264.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v267,
                                                                            args: ref v268,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v267 {
                                                                                    &Opcode::Bor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v264.0 {
                                                                                                let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                if v7.0 == v269.0 {
                                                                                                    let v837 = constructor_band(ctx, v2.0, v140.0, v269.1);
                                                                                                    let v838 = constructor_bor(ctx, v2.0, v837, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 497.
                                                                                                    returns.extend(Some(v838));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v269.1 {
                                                                                                    let v845 = constructor_band(ctx, v2.0, v140.0, v269.0);
                                                                                                    let v846 = constructor_bor(ctx, v2.0, v845, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 501.
                                                                                                    returns.extend(Some(v846));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_137
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v264.0 {
                                                                                                let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                if v7.0 == v269.0 {
                                                                                                    let v815 = constructor_band(ctx, v2.0, v269.1, v140.0);
                                                                                                    let v816 = constructor_bor(ctx, v2.0, v815, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 441.
                                                                                                    returns.extend(Some(v816));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v269.1 {
                                                                                                    let v807 = constructor_band(ctx, v2.0, v269.0, v140.0);
                                                                                                    let v808 = constructor_bor(ctx, v2.0, v807, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 437.
                                                                                                    returns.extend(Some(v808));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_138
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_136
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v632,
                                                                            arg: v633,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v632 {
                                                                                if v7.0 == v633 {
                                                                                    if v2.0 == v264.0 {
                                                                                        let v790 = constructor_bor(ctx, v2.0, v7.0, v140.0);
                                                                                        // Rule at src/opts/bitops.isle line 534.
                                                                                        returns.extend(Some(v790));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                let mut v279 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                let mut v279 = v279.into_context_iter();
                                                                while let Some(v280) = v279.next(ctx) {
                                                                    match &v280.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v283,
                                                                            args: ref v284,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v283 {
                                                                                    &Opcode::Bor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v280.0 {
                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                if v7.0 == v285.0 {
                                                                                                    let v841 = constructor_band(ctx, v2.0, v140.1, v285.1);
                                                                                                    let v842 = constructor_bor(ctx, v2.0, v841, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 499.
                                                                                                    returns.extend(Some(v842));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v285.1 {
                                                                                                    let v849 = constructor_band(ctx, v2.0, v140.1, v285.0);
                                                                                                    let v850 = constructor_bor(ctx, v2.0, v849, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 503.
                                                                                                    returns.extend(Some(v850));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_140
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v280.0 {
                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                if v7.0 == v285.0 {
                                                                                                    let v811 = constructor_band(ctx, v2.0, v285.1, v140.1);
                                                                                                    let v812 = constructor_bor(ctx, v2.0, v811, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 439.
                                                                                                    returns.extend(Some(v812));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v285.1 {
                                                                                                    let v803 = constructor_band(ctx, v2.0, v285.0, v140.1);
                                                                                                    let v804 = constructor_bor(ctx, v2.0, v803, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 435.
                                                                                                    returns.extend(Some(v804));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_141
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_139
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v635,
                                                                            arg: v636,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v635 {
                                                                                if v7.0 == v636 {
                                                                                    if v2.0 == v280.0 {
                                                                                        let v627 = constructor_bor(ctx, v2.0, v7.0, v140.1);
                                                                                        // Rule at src/opts/bitops.isle line 532.
                                                                                        returns.extend(Some(v627));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                if v7.0 == v140.0 {
                                                                    // Rule at src/opts/bitops.isle line 213.
                                                                    returns.extend(Some(v7.0));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                            }
                                                            false
                                                        }
                                                        )() { return true; } // __isle_arm_129
                                                    }
                                                    &Opcode::Bor => {
                                                        if (|| -> bool {
                                                            if v2.0 == v11.0 {
                                                                let mut v18 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                let mut v18 = v18.into_context_iter();
                                                                while let Some(v19) = v18.next(ctx) {
                                                                    if let &InstructionData::Binary {
                                                                        opcode: ref v102,
                                                                        args: ref v103,
                                                                    } = &v19.1 {
                                                                        if let &Opcode::Bxor = v102 {
                                                                            if v2.0 == v19.0 {
                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                if v104.0 == v140.0 {
                                                                                    if v104.1 == v140.1 {
                                                                                        let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                        // Rule at src/opts/bitops.isle line 615.
                                                                                        returns.extend(Some(v245));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                        // Rule at src/opts/bitops.isle line 621.
                                                                                        returns.extend(Some(v246));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                    let v699 = constructor_bor(ctx, v2.0, v140.1, v104.0);
                                                                                    let v795 = constructor_bor(ctx, v2.0, v699, v104.1);
                                                                                    // Rule at src/opts/bitops.isle line 416.
                                                                                    returns.extend(Some(v795));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    let v693 = constructor_bor(ctx, v2.0, v104.0, v140.1);
                                                                                    let v817 = constructor_bor(ctx, v2.0, v693, v104.1);
                                                                                    // Rule at src/opts/bitops.isle line 445.
                                                                                    returns.extend(Some(v817));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v104.0 == v140.1 {
                                                                                    if v104.1 == v140.0 {
                                                                                        let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                        // Rule at src/opts/bitops.isle line 617.
                                                                                        returns.extend(Some(v246));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                        // Rule at src/opts/bitops.isle line 619.
                                                                                        returns.extend(Some(v245));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                    let v691 = constructor_bor(ctx, v2.0, v140.0, v104.0);
                                                                                    let v793 = constructor_bor(ctx, v2.0, v691, v104.1);
                                                                                    // Rule at src/opts/bitops.isle line 414.
                                                                                    returns.extend(Some(v793));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    let v689 = constructor_bor(ctx, v2.0, v104.0, v140.0);
                                                                                    let v819 = constructor_bor(ctx, v2.0, v689, v104.1);
                                                                                    // Rule at src/opts/bitops.isle line 449.
                                                                                    returns.extend(Some(v819));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v104.1 == v140.0 {
                                                                                    let v703 = constructor_bor(ctx, v2.0, v140.1, v104.1);
                                                                                    let v799 = constructor_bor(ctx, v2.0, v703, v104.0);
                                                                                    // Rule at src/opts/bitops.isle line 420.
                                                                                    returns.extend(Some(v799));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    let v701 = constructor_bor(ctx, v2.0, v104.1, v140.1);
                                                                                    let v818 = constructor_bor(ctx, v2.0, v701, v104.0);
                                                                                    // Rule at src/opts/bitops.isle line 447.
                                                                                    returns.extend(Some(v818));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v104.1 == v140.1 {
                                                                                    let v695 = constructor_bor(ctx, v2.0, v140.0, v104.1);
                                                                                    let v797 = constructor_bor(ctx, v2.0, v695, v104.0);
                                                                                    // Rule at src/opts/bitops.isle line 418.
                                                                                    returns.extend(Some(v797));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    let v697 = constructor_bor(ctx, v2.0, v104.1, v140.0);
                                                                                    let v820 = constructor_bor(ctx, v2.0, v697, v104.0);
                                                                                    // Rule at src/opts/bitops.isle line 451.
                                                                                    returns.extend(Some(v820));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                if v7.0 == v140.0 {
                                                                    let v627 = constructor_bor(ctx, v2.0, v7.0, v140.1);
                                                                    // Rule at src/opts/bitops.isle line 189.
                                                                    returns.extend(Some(v627));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                                if v7.0 == v140.1 {
                                                                    let v355 = constructor_bor(ctx, v2.0, v140.0, v7.0);
                                                                    // Rule at src/opts/bitops.isle line 190.
                                                                    returns.extend(Some(v355));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                            }
                                                            match v2.0 {
                                                                I32 => {
                                                                    if (|| -> bool {
                                                                        if v11.0 == I32 {
                                                                            let mut v18 = C::inst_data_value_etor_returns::default();
                                                                            C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                            let mut v18 = v18.into_context_iter();
                                                                            while let Some(v19) = v18.next(ctx) {
                                                                                if v19.0 == I32 {
                                                                                    if let &InstructionData::Binary {
                                                                                        opcode: ref v102,
                                                                                        args: ref v103,
                                                                                    } = &v19.1 {
                                                                                        if let &Opcode::Bor = v102 {
                                                                                            let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                            let mut v107 = C::inst_data_value_etor_returns::default();
                                                                                            C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                                            let mut v107 = v107.into_context_iter();
                                                                                            while let Some(v108) = v107.next(ctx) {
                                                                                                if v108.0 == I32 {
                                                                                                    if let &InstructionData::Binary {
                                                                                                        opcode: ref v289,
                                                                                                        args: ref v290,
                                                                                                    } = &v108.1 {
                                                                                                        if let &Opcode::Ishl = v289 {
                                                                                                            let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                            C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                            let mut v114 = v114.into_context_iter();
                                                                                                            while let Some(v115) = v114.next(ctx) {
                                                                                                                if v115.0 == I32 {
                                                                                                                    if let &InstructionData::Binary {
                                                                                                                        opcode: ref v273,
                                                                                                                        args: ref v274,
                                                                                                                    } = &v115.1 {
                                                                                                                        if let &Opcode::Ishl = v273 {
                                                                                                                            let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                            let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                                            C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                                            let mut v263 = v263.into_context_iter();
                                                                                                                            while let Some(v264) = v263.next(ctx) {
                                                                                                                                if v264.0 == I32 {
                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                        opcode: ref v267,
                                                                                                                                        args: ref v268,
                                                                                                                                    } = &v264.1 {
                                                                                                                                        if let &Opcode::Ushr = v267 {
                                                                                                                                            let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                                                            let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                                                            if v269.0 == v291.0 {
                                                                                                                                                let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                                                                C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                                                                let mut v279 = v279.into_context_iter();
                                                                                                                                                while let Some(v280) = v279.next(ctx) {
                                                                                                                                                    if v280.0 == I32 {
                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                            opcode: ref v283,
                                                                                                                                                            args: ref v284,
                                                                                                                                                        } = &v280.1 {
                                                                                                                                                            if let &Opcode::Band = v283 {
                                                                                                                                                                let mut v389 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                C::inst_data_value_etor(ctx, v291.1, &mut v389);
                                                                                                                                                                let mut v389 = v389.into_context_iter();
                                                                                                                                                                while let Some(v390) = v389.next(ctx) {
                                                                                                                                                                    if v390.0 == I32 {
                                                                                                                                                                        if let &InstructionData::UnaryImm {
                                                                                                                                                                            opcode: ref v393,
                                                                                                                                                                            imm: v394,
                                                                                                                                                                        } = &v390.1 {
                                                                                                                                                                            if let &Opcode::Iconst = v393 {
                                                                                                                                                                                let v395 = C::u64_from_imm64(ctx, v394);
                                                                                                                                                                                if v395 == 0x18_u64 {
                                                                                                                                                                                    let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                                                                                                    let mut v396 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                    C::inst_data_value_etor(ctx, v275.0, &mut v396);
                                                                                                                                                                                    let mut v396 = v396.into_context_iter();
                                                                                                                                                                                    while let Some(v397) = v396.next(ctx) {
                                                                                                                                                                                        if v397.0 == I32 {
                                                                                                                                                                                            if let &InstructionData::Binary {
                                                                                                                                                                                                opcode: ref v400,
                                                                                                                                                                                                args: ref v401,
                                                                                                                                                                                            } = &v397.1 {
                                                                                                                                                                                                if let &Opcode::Band = v400 {
                                                                                                                                                                                                    let v402 = C::unpack_value_array_2(ctx, v401);
                                                                                                                                                                                                    if v269.0 == v402.0 {
                                                                                                                                                                                                        let mut v405 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v402.1, &mut v405);
                                                                                                                                                                                                        let mut v405 = v405.into_context_iter();
                                                                                                                                                                                                        while let Some(v406) = v405.next(ctx) {
                                                                                                                                                                                                            if v406.0 == I32 {
                                                                                                                                                                                                                if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                    opcode: ref v409,
                                                                                                                                                                                                                    imm: v410,
                                                                                                                                                                                                                } = &v406.1 {
                                                                                                                                                                                                                    if let &Opcode::Iconst = v409 {
                                                                                                                                                                                                                        let v411 = C::u64_from_imm64(ctx, v410);
                                                                                                                                                                                                                        if v411 == 0xff00_u64 {
                                                                                                                                                                                                                            let mut v412 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v275.1, &mut v412);
                                                                                                                                                                                                                            let mut v412 = v412.into_context_iter();
                                                                                                                                                                                                                            while let Some(v413) = v412.next(ctx) {
                                                                                                                                                                                                                                if v413.0 == I32 {
                                                                                                                                                                                                                                    if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                        opcode: ref v416,
                                                                                                                                                                                                                                        imm: v417,
                                                                                                                                                                                                                                    } = &v413.1 {
                                                                                                                                                                                                                                        if let &Opcode::Iconst = v416 {
                                                                                                                                                                                                                                            let v418 = C::u64_from_imm64(ctx, v417);
                                                                                                                                                                                                                                            if v418 == 0x8_u64 {
                                                                                                                                                                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                                                                                                                                                                let mut v419 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v285.0, &mut v419);
                                                                                                                                                                                                                                                let mut v419 = v419.into_context_iter();
                                                                                                                                                                                                                                                while let Some(v420) = v419.next(ctx) {
                                                                                                                                                                                                                                                    if v420.0 == I32 {
                                                                                                                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                                                                                                                            opcode: ref v423,
                                                                                                                                                                                                                                                            args: ref v424,
                                                                                                                                                                                                                                                        } = &v420.1 {
                                                                                                                                                                                                                                                            if let &Opcode::Ushr = v423 {
                                                                                                                                                                                                                                                                let v425 = C::unpack_value_array_2(ctx, v424);
                                                                                                                                                                                                                                                                if v269.0 == v425.0 {
                                                                                                                                                                                                                                                                    let mut v428 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v425.1, &mut v428);
                                                                                                                                                                                                                                                                    let mut v428 = v428.into_context_iter();
                                                                                                                                                                                                                                                                    while let Some(v429) = v428.next(ctx) {
                                                                                                                                                                                                                                                                        if v429.0 == I32 {
                                                                                                                                                                                                                                                                            if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                opcode: ref v432,
                                                                                                                                                                                                                                                                                imm: v433,
                                                                                                                                                                                                                                                                            } = &v429.1 {
                                                                                                                                                                                                                                                                                if let &Opcode::Iconst = v432 {
                                                                                                                                                                                                                                                                                    let v434 = C::u64_from_imm64(ctx, v433);
                                                                                                                                                                                                                                                                                    if v434 == 0x8_u64 {
                                                                                                                                                                                                                                                                                        let mut v435 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v285.1, &mut v435);
                                                                                                                                                                                                                                                                                        let mut v435 = v435.into_context_iter();
                                                                                                                                                                                                                                                                                        while let Some(v436) = v435.next(ctx) {
                                                                                                                                                                                                                                                                                            if v436.0 == I32 {
                                                                                                                                                                                                                                                                                                if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                    opcode: ref v439,
                                                                                                                                                                                                                                                                                                    imm: v440,
                                                                                                                                                                                                                                                                                                } = &v436.1 {
                                                                                                                                                                                                                                                                                                    if let &Opcode::Iconst = v439 {
                                                                                                                                                                                                                                                                                                        let v441 = C::u64_from_imm64(ctx, v440);
                                                                                                                                                                                                                                                                                                        if v441 == 0xff00_u64 {
                                                                                                                                                                                                                                                                                                            let mut v442 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v269.1, &mut v442);
                                                                                                                                                                                                                                                                                                            let mut v442 = v442.into_context_iter();
                                                                                                                                                                                                                                                                                                            while let Some(v443) = v442.next(ctx) {
                                                                                                                                                                                                                                                                                                                if v443.0 == I32 {
                                                                                                                                                                                                                                                                                                                    if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                        opcode: ref v446,
                                                                                                                                                                                                                                                                                                                        imm: v447,
                                                                                                                                                                                                                                                                                                                    } = &v443.1 {
                                                                                                                                                                                                                                                                                                                        if let &Opcode::Iconst = v446 {
                                                                                                                                                                                                                                                                                                                            let v448 = C::u64_from_imm64(ctx, v447);
                                                                                                                                                                                                                                                                                                                            if v448 == 0x18_u64 {
                                                                                                                                                                                                                                                                                                                                let v449 = constructor_bswap(ctx, v2.0, v291.0);
                                                                                                                                                                                                                                                                                                                                // Rule at src/opts/bitops.isle line 142.
                                                                                                                                                                                                                                                                                                                                returns.extend(Some(v449));
                                                                                                                                                                                                                                                                                                                                if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        false
                                                                    }
                                                                    )() { return true; } // __isle_arm_143
                                                                }
                                                                I64 => {
                                                                    if (|| -> bool {
                                                                        if v11.0 == I64 {
                                                                            let mut v18 = C::inst_data_value_etor_returns::default();
                                                                            C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                            let mut v18 = v18.into_context_iter();
                                                                            while let Some(v19) = v18.next(ctx) {
                                                                                if v19.0 == I64 {
                                                                                    if let &InstructionData::Binary {
                                                                                        opcode: ref v102,
                                                                                        args: ref v103,
                                                                                    } = &v19.1 {
                                                                                        if let &Opcode::Bor = v102 {
                                                                                            let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                            let mut v107 = C::inst_data_value_etor_returns::default();
                                                                                            C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                                            let mut v107 = v107.into_context_iter();
                                                                                            while let Some(v108) = v107.next(ctx) {
                                                                                                if v108.0 == I64 {
                                                                                                    if let &InstructionData::Binary {
                                                                                                        opcode: ref v289,
                                                                                                        args: ref v290,
                                                                                                    } = &v108.1 {
                                                                                                        if let &Opcode::Bor = v289 {
                                                                                                            let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                            C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                            let mut v114 = v114.into_context_iter();
                                                                                                            while let Some(v115) = v114.next(ctx) {
                                                                                                                if v115.0 == I64 {
                                                                                                                    if let &InstructionData::Binary {
                                                                                                                        opcode: ref v273,
                                                                                                                        args: ref v274,
                                                                                                                    } = &v115.1 {
                                                                                                                        if let &Opcode::Bor = v273 {
                                                                                                                            let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                            let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                                            C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                                            let mut v263 = v263.into_context_iter();
                                                                                                                            while let Some(v264) = v263.next(ctx) {
                                                                                                                                if v264.0 == I64 {
                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                        opcode: ref v267,
                                                                                                                                        args: ref v268,
                                                                                                                                    } = &v264.1 {
                                                                                                                                        if let &Opcode::Bor = v267 {
                                                                                                                                            let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                                                            C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                                                            let mut v279 = v279.into_context_iter();
                                                                                                                                            while let Some(v280) = v279.next(ctx) {
                                                                                                                                                if v280.0 == I64 {
                                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                                        opcode: ref v283,
                                                                                                                                                        args: ref v284,
                                                                                                                                                    } = &v280.1 {
                                                                                                                                                        if let &Opcode::Bor = v283 {
                                                                                                                                                            let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                                                                            let mut v389 = C::inst_data_value_etor_returns::default();
                                                                                                                                                            C::inst_data_value_etor(ctx, v291.1, &mut v389);
                                                                                                                                                            let mut v389 = v389.into_context_iter();
                                                                                                                                                            while let Some(v390) = v389.next(ctx) {
                                                                                                                                                                if v390.0 == I64 {
                                                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                                                        opcode: ref v466,
                                                                                                                                                                        args: ref v467,
                                                                                                                                                                    } = &v390.1 {
                                                                                                                                                                        if let &Opcode::Ishl = v466 {
                                                                                                                                                                            let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                                                                                            let mut v396 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                            C::inst_data_value_etor(ctx, v275.0, &mut v396);
                                                                                                                                                                            let mut v396 = v396.into_context_iter();
                                                                                                                                                                            while let Some(v397) = v396.next(ctx) {
                                                                                                                                                                                if v397.0 == I64 {
                                                                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                                                                        opcode: ref v400,
                                                                                                                                                                                        args: ref v401,
                                                                                                                                                                                    } = &v397.1 {
                                                                                                                                                                                        if let &Opcode::Ishl = v400 {
                                                                                                                                                                                            let v402 = C::unpack_value_array_2(ctx, v401);
                                                                                                                                                                                            let mut v405 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                            C::inst_data_value_etor(ctx, v402.1, &mut v405);
                                                                                                                                                                                            let mut v405 = v405.into_context_iter();
                                                                                                                                                                                            while let Some(v406) = v405.next(ctx) {
                                                                                                                                                                                                if v406.0 == I64 {
                                                                                                                                                                                                    if let &InstructionData::UnaryImm {
                                                                                                                                                                                                        opcode: ref v409,
                                                                                                                                                                                                        imm: v410,
                                                                                                                                                                                                    } = &v406.1 {
                                                                                                                                                                                                        if let &Opcode::Iconst = v409 {
                                                                                                                                                                                                            let v411 = C::u64_from_imm64(ctx, v410);
                                                                                                                                                                                                            if v411 == 0x18_u64 {
                                                                                                                                                                                                                let mut v412 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v275.1, &mut v412);
                                                                                                                                                                                                                let mut v412 = v412.into_context_iter();
                                                                                                                                                                                                                while let Some(v413) = v412.next(ctx) {
                                                                                                                                                                                                                    if v413.0 == I64 {
                                                                                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                                                                                            opcode: ref v510,
                                                                                                                                                                                                                            args: ref v511,
                                                                                                                                                                                                                        } = &v413.1 {
                                                                                                                                                                                                                            if let &Opcode::Ishl = v510 {
                                                                                                                                                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                                                                                                                                                let mut v419 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v285.0, &mut v419);
                                                                                                                                                                                                                                let mut v419 = v419.into_context_iter();
                                                                                                                                                                                                                                while let Some(v420) = v419.next(ctx) {
                                                                                                                                                                                                                                    if v420.0 == I64 {
                                                                                                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                                                                                                            opcode: ref v423,
                                                                                                                                                                                                                                            args: ref v424,
                                                                                                                                                                                                                                        } = &v420.1 {
                                                                                                                                                                                                                                            if let &Opcode::Band = v423 {
                                                                                                                                                                                                                                                let v425 = C::unpack_value_array_2(ctx, v424);
                                                                                                                                                                                                                                                let mut v428 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v425.1, &mut v428);
                                                                                                                                                                                                                                                let mut v428 = v428.into_context_iter();
                                                                                                                                                                                                                                                while let Some(v429) = v428.next(ctx) {
                                                                                                                                                                                                                                                    if v429.0 == I64 {
                                                                                                                                                                                                                                                        if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                            opcode: ref v432,
                                                                                                                                                                                                                                                            imm: v433,
                                                                                                                                                                                                                                                        } = &v429.1 {
                                                                                                                                                                                                                                                            if let &Opcode::Iconst = v432 {
                                                                                                                                                                                                                                                                let v434 = C::u64_from_imm64(ctx, v433);
                                                                                                                                                                                                                                                                if v434 == 0xff000000_u64 {
                                                                                                                                                                                                                                                                    let mut v435 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v285.1, &mut v435);
                                                                                                                                                                                                                                                                    let mut v435 = v435.into_context_iter();
                                                                                                                                                                                                                                                                    while let Some(v436) = v435.next(ctx) {
                                                                                                                                                                                                                                                                        if v436.0 == I64 {
                                                                                                                                                                                                                                                                            if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                opcode: ref v554,
                                                                                                                                                                                                                                                                                args: ref v555,
                                                                                                                                                                                                                                                                            } = &v436.1 {
                                                                                                                                                                                                                                                                                if let &Opcode::Band = v554 {
                                                                                                                                                                                                                                                                                    let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                                                                                                                                                                                                    let mut v442 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v269.1, &mut v442);
                                                                                                                                                                                                                                                                                    let mut v442 = v442.into_context_iter();
                                                                                                                                                                                                                                                                                    while let Some(v443) = v442.next(ctx) {
                                                                                                                                                                                                                                                                                        if v443.0 == I64 {
                                                                                                                                                                                                                                                                                            if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                opcode: ref v614,
                                                                                                                                                                                                                                                                                                args: ref v615,
                                                                                                                                                                                                                                                                                            } = &v443.1 {
                                                                                                                                                                                                                                                                                                if let &Opcode::Ushr = v614 {
                                                                                                                                                                                                                                                                                                    let mut v450 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v291.0, &mut v450);
                                                                                                                                                                                                                                                                                                    let mut v450 = v450.into_context_iter();
                                                                                                                                                                                                                                                                                                    while let Some(v451) = v450.next(ctx) {
                                                                                                                                                                                                                                                                                                        if v451.0 == I64 {
                                                                                                                                                                                                                                                                                                            if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                opcode: ref v454,
                                                                                                                                                                                                                                                                                                                args: ref v455,
                                                                                                                                                                                                                                                                                                            } = &v451.1 {
                                                                                                                                                                                                                                                                                                                if let &Opcode::Ishl = v454 {
                                                                                                                                                                                                                                                                                                                    let v456 = C::unpack_value_array_2(ctx, v455);
                                                                                                                                                                                                                                                                                                                    let v616 = C::unpack_value_array_2(ctx, v615);
                                                                                                                                                                                                                                                                                                                    if v456.0 == v616.0 {
                                                                                                                                                                                                                                                                                                                        let mut v459 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v456.1, &mut v459);
                                                                                                                                                                                                                                                                                                                        let mut v459 = v459.into_context_iter();
                                                                                                                                                                                                                                                                                                                        while let Some(v460) = v459.next(ctx) {
                                                                                                                                                                                                                                                                                                                            if v460.0 == I64 {
                                                                                                                                                                                                                                                                                                                                if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                    opcode: ref v463,
                                                                                                                                                                                                                                                                                                                                    imm: v464,
                                                                                                                                                                                                                                                                                                                                } = &v460.1 {
                                                                                                                                                                                                                                                                                                                                    if let &Opcode::Iconst = v463 {
                                                                                                                                                                                                                                                                                                                                        let v465 = C::u64_from_imm64(ctx, v464);
                                                                                                                                                                                                                                                                                                                                        if v465 == 0x38_u64 {
                                                                                                                                                                                                                                                                                                                                            let v468 = C::unpack_value_array_2(ctx, v467);
                                                                                                                                                                                                                                                                                                                                            let mut v471 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v468.0, &mut v471);
                                                                                                                                                                                                                                                                                                                                            let mut v471 = v471.into_context_iter();
                                                                                                                                                                                                                                                                                                                                            while let Some(v472) = v471.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                if v472.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                        opcode: ref v475,
                                                                                                                                                                                                                                                                                                                                                        args: ref v476,
                                                                                                                                                                                                                                                                                                                                                    } = &v472.1 {
                                                                                                                                                                                                                                                                                                                                                        if let &Opcode::Band = v475 {
                                                                                                                                                                                                                                                                                                                                                            let v477 = C::unpack_value_array_2(ctx, v476);
                                                                                                                                                                                                                                                                                                                                                            if v456.0 == v477.0 {
                                                                                                                                                                                                                                                                                                                                                                let mut v480 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v477.1, &mut v480);
                                                                                                                                                                                                                                                                                                                                                                let mut v480 = v480.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                while let Some(v481) = v480.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                    if v481.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                        if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                            opcode: ref v484,
                                                                                                                                                                                                                                                                                                                                                                            imm: v485,
                                                                                                                                                                                                                                                                                                                                                                        } = &v481.1 {
                                                                                                                                                                                                                                                                                                                                                                            if let &Opcode::Iconst = v484 {
                                                                                                                                                                                                                                                                                                                                                                                let v486 = C::u64_from_imm64(ctx, v485);
                                                                                                                                                                                                                                                                                                                                                                                if v486 == 0xff00_u64 {
                                                                                                                                                                                                                                                                                                                                                                                    let mut v487 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v468.1, &mut v487);
                                                                                                                                                                                                                                                                                                                                                                                    let mut v487 = v487.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                    while let Some(v488) = v487.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                        if v488.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                            if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                opcode: ref v491,
                                                                                                                                                                                                                                                                                                                                                                                                imm: v492,
                                                                                                                                                                                                                                                                                                                                                                                            } = &v488.1 {
                                                                                                                                                                                                                                                                                                                                                                                                if let &Opcode::Iconst = v491 {
                                                                                                                                                                                                                                                                                                                                                                                                    let v493 = C::u64_from_imm64(ctx, v492);
                                                                                                                                                                                                                                                                                                                                                                                                    if v493 == 0x28_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                        let mut v494 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v402.0, &mut v494);
                                                                                                                                                                                                                                                                                                                                                                                                        let mut v494 = v494.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                        while let Some(v495) = v494.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                            if v495.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                                                                                    opcode: ref v498,
                                                                                                                                                                                                                                                                                                                                                                                                                    args: ref v499,
                                                                                                                                                                                                                                                                                                                                                                                                                } = &v495.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                    if let &Opcode::Band = v498 {
                                                                                                                                                                                                                                                                                                                                                                                                                        let v500 = C::unpack_value_array_2(ctx, v499);
                                                                                                                                                                                                                                                                                                                                                                                                                        if v456.0 == v500.0 {
                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v503 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v500.1, &mut v503);
                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v503 = v503.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                            while let Some(v504) = v503.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                if v504.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                        opcode: ref v507,
                                                                                                                                                                                                                                                                                                                                                                                                                                        imm: v508,
                                                                                                                                                                                                                                                                                                                                                                                                                                    } = &v504.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &Opcode::Iconst = v507 {
                                                                                                                                                                                                                                                                                                                                                                                                                                            let v509 = C::u64_from_imm64(ctx, v508);
                                                                                                                                                                                                                                                                                                                                                                                                                                            if v509 == 0xff0000_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                let v512 = C::unpack_value_array_2(ctx, v511);
                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v515 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v512.0, &mut v515);
                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v515 = v515.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                while let Some(v516) = v515.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v516.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            opcode: ref v519,
                                                                                                                                                                                                                                                                                                                                                                                                                                                            args: ref v520,
                                                                                                                                                                                                                                                                                                                                                                                                                                                        } = &v516.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &Opcode::Band = v519 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let v521 = C::unpack_value_array_2(ctx, v520);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                if v456.0 == v521.0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let mut v524 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v521.1, &mut v524);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let mut v524 = v524.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    while let Some(v525) = v524.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if v525.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                opcode: ref v528,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                imm: v529,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            } = &v525.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if let &Opcode::Iconst = v528 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let v530 = C::u64_from_imm64(ctx, v529);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v530 == 0xff000000_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut v531 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v512.1, &mut v531);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut v531 = v531.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        while let Some(v532) = v531.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if v532.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    opcode: ref v535,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    imm: v536,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                } = &v532.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &Opcode::Iconst = v535 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let v537 = C::u64_from_imm64(ctx, v536);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if v537 == 0x8_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v538 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v425.0, &mut v538);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v538 = v538.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            while let Some(v539) = v538.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if v539.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        opcode: ref v542,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        args: ref v543,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    } = &v539.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &Opcode::Ushr = v542 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let v544 = C::unpack_value_array_2(ctx, v543);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if v456.0 == v544.0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v547 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v544.1, &mut v547);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v547 = v547.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                while let Some(v548) = v547.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v548.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            opcode: ref v551,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            imm: v552,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        } = &v548.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &Opcode::Iconst = v551 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let v553 = C::u64_from_imm64(ctx, v552);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if v553 == 0x8_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let v556 = C::unpack_value_array_2(ctx, v555);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let mut v559 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v556.0, &mut v559);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let mut v559 = v559.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    while let Some(v560) = v559.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if v560.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                opcode: ref v563,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                args: ref v564,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            } = &v560.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if let &Opcode::Ushr = v563 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let v565 = C::unpack_value_array_2(ctx, v564);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v456.0 == v565.0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut v568 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v565.1, &mut v568);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut v568 = v568.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        while let Some(v569) = v568.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if v569.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    opcode: ref v572,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    imm: v573,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                } = &v569.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &Opcode::Iconst = v572 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let v574 = C::u64_from_imm64(ctx, v573);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if v574 == 0x18_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v575 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v556.1, &mut v575);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v575 = v575.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            while let Some(v576) = v575.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if v576.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        opcode: ref v579,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        imm: v580,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    } = &v576.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &Opcode::Iconst = v579 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let v581 = C::u64_from_imm64(ctx, v580);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if v581 == 0xff0000_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v582 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v269.0, &mut v582);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v582 = v582.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                while let Some(v583) = v582.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v583.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            opcode: ref v586,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            args: ref v587,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        } = &v583.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &Opcode::Band = v586 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let v588 = C::unpack_value_array_2(ctx, v587);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v591 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                C::inst_data_value_etor(ctx, v588.0, &mut v591);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut v591 = v591.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                while let Some(v592) = v591.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v592.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &InstructionData::Binary {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            opcode: ref v595,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            args: ref v596,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        } = &v592.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &Opcode::Ushr = v595 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let v597 = C::unpack_value_array_2(ctx, v596);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if v456.0 == v597.0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let mut v600 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    C::inst_data_value_etor(ctx, v597.1, &mut v600);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let mut v600 = v600.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    while let Some(v601) = v600.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if v601.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                opcode: ref v604,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                imm: v605,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            } = &v601.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if let &Opcode::Iconst = v604 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    let v606 = C::u64_from_imm64(ctx, v605);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if v606 == 0x28_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut v607 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        C::inst_data_value_etor(ctx, v588.1, &mut v607);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut v607 = v607.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        while let Some(v608) = v607.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if v608.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    opcode: ref v611,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    imm: v612,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                } = &v608.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &Opcode::Iconst = v611 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let v613 = C::u64_from_imm64(ctx, v612);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if v613 == 0xff00_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v619 = C::inst_data_value_etor_returns::default();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            C::inst_data_value_etor(ctx, v616.1, &mut v619);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let mut v619 = v619.into_context_iter();
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            while let Some(v620) = v619.next(ctx) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if v620.0 == I64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    if let &InstructionData::UnaryImm {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        opcode: ref v623,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        imm: v624,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    } = &v620.1 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let &Opcode::Iconst = v623 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            let v625 = C::u64_from_imm64(ctx, v624);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            if v625 == 0x38_u64 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let v626 = constructor_bswap(ctx, v2.0, v456.0);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                // Rule at src/opts/bitops.isle line 155.
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                returns.extend(Some(v626));
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        false
                                                                    }
                                                                    )() { return true; } // __isle_arm_144
                                                                }
                                                                _ => {}
                                                            }
                                                            false
                                                        }
                                                        )() { return true; } // __isle_arm_142
                                                    }
                                                    &Opcode::Bxor => {
                                                        if (|| -> bool {
                                                            if v2.0 == v11.0 {
                                                                let mut v18 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                let mut v18 = v18.into_context_iter();
                                                                while let Some(v19) = v18.next(ctx) {
                                                                    match &v19.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v102,
                                                                            args: ref v103,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v102 {
                                                                                    &Opcode::Band => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                let mut v107 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                                                let mut v107 = v107.into_context_iter();
                                                                                                while let Some(v108) = v107.next(ctx) {
                                                                                                    if let &InstructionData::Unary {
                                                                                                        opcode: ref v111,
                                                                                                        arg: v112,
                                                                                                    } = &v108.1 {
                                                                                                        if let &Opcode::Bnot = v111 {
                                                                                                            if v2.0 == v108.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                if v104.1 == v140.0 {
                                                                                                                    if v112 == v140.1 {
                                                                                                                        let v875 = constructor_bxor(ctx, v2.0, v104.1, v112);
                                                                                                                        // Rule at src/opts/bitops.isle line 562.
                                                                                                                        returns.extend(Some(v875));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v112 == v140.0 {
                                                                                                                    if v104.1 == v140.1 {
                                                                                                                        let v875 = constructor_bxor(ctx, v2.0, v104.1, v112);
                                                                                                                        // Rule at src/opts/bitops.isle line 564.
                                                                                                                        returns.extend(Some(v875));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                let mut v114 = v114.into_context_iter();
                                                                                                while let Some(v115) = v114.next(ctx) {
                                                                                                    if let &InstructionData::Unary {
                                                                                                        opcode: ref v118,
                                                                                                        arg: v119,
                                                                                                    } = &v115.1 {
                                                                                                        if let &Opcode::Bnot = v118 {
                                                                                                            if v2.0 == v115.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                if v104.0 == v140.0 {
                                                                                                                    if v119 == v140.1 {
                                                                                                                        let v873 = constructor_bxor(ctx, v2.0, v104.0, v119);
                                                                                                                        // Rule at src/opts/bitops.isle line 558.
                                                                                                                        returns.extend(Some(v873));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v119 == v140.0 {
                                                                                                                    if v104.0 == v140.1 {
                                                                                                                        let v873 = constructor_bxor(ctx, v2.0, v104.0, v119);
                                                                                                                        // Rule at src/opts/bitops.isle line 560.
                                                                                                                        returns.extend(Some(v873));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_147
                                                                                    }
                                                                                    &Opcode::Bor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                if v104.0 == v140.0 {
                                                                                                    if v104.1 == v140.1 {
                                                                                                        let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                                        // Rule at src/opts/bitops.isle line 614.
                                                                                                        returns.extend(Some(v245));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                                        // Rule at src/opts/bitops.isle line 620.
                                                                                                        returns.extend(Some(v246));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                    let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                                    let v796 = constructor_bor(ctx, v2.0, v246, v140.1);
                                                                                                    // Rule at src/opts/bitops.isle line 417.
                                                                                                    returns.extend(Some(v796));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                                    let v794 = constructor_bor(ctx, v2.0, v245, v140.1);
                                                                                                    // Rule at src/opts/bitops.isle line 444.
                                                                                                    returns.extend(Some(v794));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v104.0 == v140.1 {
                                                                                                    if v104.1 == v140.0 {
                                                                                                        let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                                        // Rule at src/opts/bitops.isle line 616.
                                                                                                        returns.extend(Some(v245));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                                        // Rule at src/opts/bitops.isle line 618.
                                                                                                        returns.extend(Some(v246));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                    let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                                    let v800 = constructor_bor(ctx, v2.0, v246, v140.0);
                                                                                                    // Rule at src/opts/bitops.isle line 421.
                                                                                                    returns.extend(Some(v800));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                                    let v798 = constructor_bor(ctx, v2.0, v245, v140.0);
                                                                                                    // Rule at src/opts/bitops.isle line 446.
                                                                                                    returns.extend(Some(v798));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v104.1 == v140.0 {
                                                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                                    let v794 = constructor_bor(ctx, v2.0, v245, v140.1);
                                                                                                    // Rule at src/opts/bitops.isle line 415.
                                                                                                    returns.extend(Some(v794));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                                    let v796 = constructor_bor(ctx, v2.0, v246, v140.1);
                                                                                                    // Rule at src/opts/bitops.isle line 448.
                                                                                                    returns.extend(Some(v796));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v104.1 == v140.1 {
                                                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                                                    let v798 = constructor_bor(ctx, v2.0, v245, v140.0);
                                                                                                    // Rule at src/opts/bitops.isle line 419.
                                                                                                    returns.extend(Some(v798));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                                                    let v800 = constructor_bor(ctx, v2.0, v246, v140.0);
                                                                                                    // Rule at src/opts/bitops.isle line 450.
                                                                                                    returns.extend(Some(v800));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_148
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                let mut v107 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                                                let mut v107 = v107.into_context_iter();
                                                                                                while let Some(v108) = v107.next(ctx) {
                                                                                                    if let &InstructionData::Binary {
                                                                                                        opcode: ref v289,
                                                                                                        args: ref v290,
                                                                                                    } = &v108.1 {
                                                                                                        if let &Opcode::Bxor = v289 {
                                                                                                            if v2.0 == v108.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                                if v140.0 == v291.0 {
                                                                                                                    if v140.1 == v291.1 {
                                                                                                                        let v779 = constructor_bxor(ctx, v2.0, v291.0, v291.1);
                                                                                                                        let v780 = constructor_bor(ctx, v2.0, v779, v104.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 384.
                                                                                                                        returns.extend(Some(v780));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v784 = constructor_bxor(ctx, v2.0, v291.1, v291.0);
                                                                                                                        let v785 = constructor_bor(ctx, v2.0, v784, v104.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 399.
                                                                                                                        returns.extend(Some(v785));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v140.0 == v291.1 {
                                                                                                                    if v140.1 == v291.0 {
                                                                                                                        let v784 = constructor_bxor(ctx, v2.0, v291.1, v291.0);
                                                                                                                        let v785 = constructor_bor(ctx, v2.0, v784, v104.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 389.
                                                                                                                        returns.extend(Some(v785));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v779 = constructor_bxor(ctx, v2.0, v291.0, v291.1);
                                                                                                                        let v780 = constructor_bor(ctx, v2.0, v779, v104.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 394.
                                                                                                                        returns.extend(Some(v780));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                let mut v114 = v114.into_context_iter();
                                                                                                while let Some(v115) = v114.next(ctx) {
                                                                                                    if let &InstructionData::Binary {
                                                                                                        opcode: ref v273,
                                                                                                        args: ref v274,
                                                                                                    } = &v115.1 {
                                                                                                        if let &Opcode::Bxor = v273 {
                                                                                                            if v2.0 == v115.0 {
                                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                                let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                                if v140.0 == v275.0 {
                                                                                                                    if v140.1 == v275.1 {
                                                                                                                        let v782 = constructor_bxor(ctx, v2.0, v275.0, v275.1);
                                                                                                                        let v783 = constructor_bor(ctx, v2.0, v782, v104.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 386.
                                                                                                                        returns.extend(Some(v783));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v786 = constructor_bxor(ctx, v2.0, v275.1, v275.0);
                                                                                                                        let v787 = constructor_bor(ctx, v2.0, v786, v104.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 401.
                                                                                                                        returns.extend(Some(v787));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v140.0 == v275.1 {
                                                                                                                    if v140.1 == v275.0 {
                                                                                                                        let v786 = constructor_bxor(ctx, v2.0, v275.1, v275.0);
                                                                                                                        let v787 = constructor_bor(ctx, v2.0, v786, v104.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 391.
                                                                                                                        returns.extend(Some(v787));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v782 = constructor_bxor(ctx, v2.0, v275.0, v275.1);
                                                                                                                        let v783 = constructor_bor(ctx, v2.0, v782, v104.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 396.
                                                                                                                        returns.extend(Some(v783));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                                let mut v263 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                                                let mut v263 = v263.into_context_iter();
                                                                                                while let Some(v264) = v263.next(ctx) {
                                                                                                    if let &InstructionData::Binary {
                                                                                                        opcode: ref v267,
                                                                                                        args: ref v268,
                                                                                                    } = &v264.1 {
                                                                                                        if let &Opcode::Bxor = v267 {
                                                                                                            if v2.0 == v264.0 {
                                                                                                                let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                                if v104.0 == v269.0 {
                                                                                                                    if v104.1 == v269.1 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v781 = constructor_bor(ctx, v2.0, v243, v140.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 385.
                                                                                                                        returns.extend(Some(v781));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v789 = constructor_bor(ctx, v2.0, v244, v140.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 400.
                                                                                                                        returns.extend(Some(v789));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v104.1 == v269.0 {
                                                                                                                    if v104.0 == v269.1 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v781 = constructor_bor(ctx, v2.0, v243, v140.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 390.
                                                                                                                        returns.extend(Some(v781));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v789 = constructor_bor(ctx, v2.0, v244, v140.0);
                                                                                                                        // Rule at src/opts/bitops.isle line 395.
                                                                                                                        returns.extend(Some(v789));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let mut v279 = C::inst_data_value_etor_returns::default();
                                                                                                C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                                                let mut v279 = v279.into_context_iter();
                                                                                                while let Some(v280) = v279.next(ctx) {
                                                                                                    if let &InstructionData::Binary {
                                                                                                        opcode: ref v283,
                                                                                                        args: ref v284,
                                                                                                    } = &v280.1 {
                                                                                                        if let &Opcode::Bxor = v283 {
                                                                                                            if v2.0 == v280.0 {
                                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                                if v104.0 == v285.0 {
                                                                                                                    if v104.1 == v285.1 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v778 = constructor_bor(ctx, v2.0, v243, v140.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 383.
                                                                                                                        returns.extend(Some(v778));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v788 = constructor_bor(ctx, v2.0, v244, v140.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 398.
                                                                                                                        returns.extend(Some(v788));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v104.1 == v285.0 {
                                                                                                                    if v104.0 == v285.1 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v778 = constructor_bor(ctx, v2.0, v243, v140.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 388.
                                                                                                                        returns.extend(Some(v778));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v788 = constructor_bor(ctx, v2.0, v244, v140.1);
                                                                                                                        // Rule at src/opts/bitops.isle line 393.
                                                                                                                        returns.extend(Some(v788));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_149
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_146
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v29,
                                                                            arg: v30,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                if let &Opcode::Bnot = v29 {
                                                                                    if v2.0 == v19.0 {
                                                                                        let v140 = C::unpack_value_array_2(ctx, v139);
                                                                                        if v30 == v140.0 {
                                                                                            let v833 = constructor_band(ctx, v2.0, v140.1, v30);
                                                                                            let v834 = constructor_bnot(ctx, v2.0, v833);
                                                                                            // Rule at src/opts/bitops.isle line 487.
                                                                                            returns.extend(Some(v834));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                        if v30 == v140.1 {
                                                                                            let v830 = constructor_band(ctx, v2.0, v140.0, v30);
                                                                                            let v831 = constructor_bnot(ctx, v2.0, v830);
                                                                                            // Rule at src/opts/bitops.isle line 485.
                                                                                            returns.extend(Some(v831));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_150
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                let v140 = C::unpack_value_array_2(ctx, v139);
                                                                let mut v263 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v140.1, &mut v263);
                                                                let mut v263 = v263.into_context_iter();
                                                                while let Some(v264) = v263.next(ctx) {
                                                                    match &v264.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v267,
                                                                            args: ref v268,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v267 {
                                                                                    &Opcode::Band => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v264.0 {
                                                                                                let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                if v7.0 == v269.0 {
                                                                                                    let v355 = constructor_bor(ctx, v2.0, v140.0, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 517.
                                                                                                    returns.extend(Some(v355));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v269.1 {
                                                                                                    let v355 = constructor_bor(ctx, v2.0, v140.0, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 521.
                                                                                                    returns.extend(Some(v355));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_152
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v264.0 {
                                                                                                let v269 = C::unpack_value_array_2(ctx, v268);
                                                                                                if v7.0 == v269.0 {
                                                                                                    let v719 = constructor_bxor(ctx, v2.0, v269.1, v140.0);
                                                                                                    let v720 = constructor_bor(ctx, v2.0, v719, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 296.
                                                                                                    returns.extend(Some(v720));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v269.1 {
                                                                                                    let v727 = constructor_bxor(ctx, v2.0, v269.0, v140.0);
                                                                                                    let v728 = constructor_bor(ctx, v2.0, v727, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 300.
                                                                                                    returns.extend(Some(v728));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_153
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_151
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v632,
                                                                            arg: v633,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v632 {
                                                                                if v7.0 == v633 {
                                                                                    if v2.0 == v264.0 {
                                                                                        let v752 = constructor_bnot(ctx, v2.0, v140.0);
                                                                                        let v891 = constructor_bor(ctx, v2.0, v7.0, v752);
                                                                                        // Rule at src/opts/bitops.isle line 598.
                                                                                        returns.extend(Some(v891));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                let mut v279 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v140.0, &mut v279);
                                                                let mut v279 = v279.into_context_iter();
                                                                while let Some(v280) = v279.next(ctx) {
                                                                    match &v280.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v283,
                                                                            args: ref v284,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v283 {
                                                                                    &Opcode::Band => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v280.0 {
                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                if v7.0 == v285.0 {
                                                                                                    let v867 = constructor_bor(ctx, v2.0, v140.1, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 519.
                                                                                                    returns.extend(Some(v867));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v285.1 {
                                                                                                    let v867 = constructor_bor(ctx, v2.0, v140.1, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 523.
                                                                                                    returns.extend(Some(v867));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_155
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v280.0 {
                                                                                                let v285 = C::unpack_value_array_2(ctx, v284);
                                                                                                if v7.0 == v285.0 {
                                                                                                    let v715 = constructor_bxor(ctx, v2.0, v285.1, v140.1);
                                                                                                    let v716 = constructor_bor(ctx, v2.0, v715, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 294.
                                                                                                    returns.extend(Some(v716));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.0 == v285.1 {
                                                                                                    let v723 = constructor_bxor(ctx, v2.0, v285.0, v140.1);
                                                                                                    let v724 = constructor_bor(ctx, v2.0, v723, v7.0);
                                                                                                    // Rule at src/opts/bitops.isle line 298.
                                                                                                    returns.extend(Some(v724));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_156
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_154
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v635,
                                                                            arg: v636,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v635 {
                                                                                if v7.0 == v636 {
                                                                                    if v2.0 == v280.0 {
                                                                                        let v749 = constructor_bnot(ctx, v2.0, v140.1);
                                                                                        let v889 = constructor_bor(ctx, v2.0, v7.0, v749);
                                                                                        // Rule at src/opts/bitops.isle line 596.
                                                                                        returns.extend(Some(v889));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                if v7.0 == v140.0 {
                                                                    let v627 = constructor_bor(ctx, v2.0, v7.0, v140.1);
                                                                    // Rule at src/opts/bitops.isle line 404.
                                                                    returns.extend(Some(v627));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                                if v7.0 == v140.1 {
                                                                    let v790 = constructor_bor(ctx, v2.0, v7.0, v140.0);
                                                                    // Rule at src/opts/bitops.isle line 406.
                                                                    returns.extend(Some(v790));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                            }
                                                            false
                                                        }
                                                        )() { return true; } // __isle_arm_145
                                                    }
                                                    _ => {}
                                                }
                                                false
                                            }
                                            )() { return true; } // __isle_arm_128
                                        }
                                        &InstructionData::IntCompare {
                                            opcode: ref v760,
                                            args: ref v761,
                                            cond: ref v762,
                                        } => {
                                            if (|| -> bool {
                                                if let &Opcode::Icmp = v760 {
                                                    match v762 {
                                                        &IntCC::Equal => {
                                                            if (|| -> bool {
                                                                if v2.0 == v11.0 {
                                                                    let mut v18 = C::inst_data_value_etor_returns::default();
                                                                    C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                    let mut v18 = v18.into_context_iter();
                                                                    while let Some(v19) = v18.next(ctx) {
                                                                        if let &InstructionData::IntCompare {
                                                                            opcode: ref v754,
                                                                            args: ref v755,
                                                                            cond: ref v756,
                                                                        } = &v19.1 {
                                                                            if let &Opcode::Icmp = v754 {
                                                                                match v756 {
                                                                                    &IntCC::SignedGreaterThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v777 = constructor_sle(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 379.
                                                                                                        returns.extend(Some(v777));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v777 = constructor_sle(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 377.
                                                                                                        returns.extend(Some(v777));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_159
                                                                                    }
                                                                                    &IntCC::SignedLessThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v776 = constructor_sle(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 371.
                                                                                                        returns.extend(Some(v776));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v776 = constructor_sle(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 373.
                                                                                                        returns.extend(Some(v776));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_160
                                                                                    }
                                                                                    &IntCC::UnsignedGreaterThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v775 = constructor_ule(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 367.
                                                                                                        returns.extend(Some(v775));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v775 = constructor_ule(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 365.
                                                                                                        returns.extend(Some(v775));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_161
                                                                                    }
                                                                                    &IntCC::UnsignedLessThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v774 = constructor_ule(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 359.
                                                                                                        returns.extend(Some(v774));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v774 = constructor_ule(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 361.
                                                                                                        returns.extend(Some(v774));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_162
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                false
                                                            }
                                                            )() { return true; } // __isle_arm_158
                                                        }
                                                        &IntCC::SignedGreaterThan => {
                                                            if (|| -> bool {
                                                                if v2.0 == v11.0 {
                                                                    let mut v18 = C::inst_data_value_etor_returns::default();
                                                                    C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                    let mut v18 = v18.into_context_iter();
                                                                    while let Some(v19) = v18.next(ctx) {
                                                                        if let &InstructionData::IntCompare {
                                                                            opcode: ref v754,
                                                                            args: ref v755,
                                                                            cond: ref v756,
                                                                        } = &v19.1 {
                                                                            if let &Opcode::Icmp = v754 {
                                                                                match v756 {
                                                                                    &IntCC::Equal => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v777 = constructor_sle(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 380.
                                                                                                        returns.extend(Some(v777));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v776 = constructor_sle(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 378.
                                                                                                        returns.extend(Some(v776));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_164
                                                                                    }
                                                                                    &IntCC::SignedGreaterThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 335.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 336.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_165
                                                                                    }
                                                                                    &IntCC::SignedLessThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 331.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 338.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_166
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                false
                                                            }
                                                            )() { return true; } // __isle_arm_163
                                                        }
                                                        &IntCC::SignedLessThan => {
                                                            if (|| -> bool {
                                                                if v2.0 == v11.0 {
                                                                    let mut v18 = C::inst_data_value_etor_returns::default();
                                                                    C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                    let mut v18 = v18.into_context_iter();
                                                                    while let Some(v19) = v18.next(ctx) {
                                                                        if let &InstructionData::IntCompare {
                                                                            opcode: ref v754,
                                                                            args: ref v755,
                                                                            cond: ref v756,
                                                                        } = &v19.1 {
                                                                            if let &Opcode::Icmp = v754 {
                                                                                match v756 {
                                                                                    &IntCC::Equal => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v776 = constructor_sle(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 372.
                                                                                                        returns.extend(Some(v776));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v777 = constructor_sle(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 374.
                                                                                                        returns.extend(Some(v777));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_168
                                                                                    }
                                                                                    &IntCC::SignedGreaterThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 332.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 337.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_169
                                                                                    }
                                                                                    &IntCC::SignedLessThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 333.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 334.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_170
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                false
                                                            }
                                                            )() { return true; } // __isle_arm_167
                                                        }
                                                        &IntCC::UnsignedGreaterThan => {
                                                            if (|| -> bool {
                                                                if v2.0 == v11.0 {
                                                                    let mut v18 = C::inst_data_value_etor_returns::default();
                                                                    C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                    let mut v18 = v18.into_context_iter();
                                                                    while let Some(v19) = v18.next(ctx) {
                                                                        if let &InstructionData::IntCompare {
                                                                            opcode: ref v754,
                                                                            args: ref v755,
                                                                            cond: ref v756,
                                                                        } = &v19.1 {
                                                                            if let &Opcode::Icmp = v754 {
                                                                                match v756 {
                                                                                    &IntCC::Equal => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v775 = constructor_ule(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 368.
                                                                                                        returns.extend(Some(v775));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v774 = constructor_ule(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 366.
                                                                                                        returns.extend(Some(v774));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_172
                                                                                    }
                                                                                    &IntCC::UnsignedGreaterThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 345.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 346.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_173
                                                                                    }
                                                                                    &IntCC::UnsignedLessThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 341.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 348.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_174
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                false
                                                            }
                                                            )() { return true; } // __isle_arm_171
                                                        }
                                                        &IntCC::UnsignedLessThan => {
                                                            if (|| -> bool {
                                                                if v2.0 == v11.0 {
                                                                    let mut v18 = C::inst_data_value_etor_returns::default();
                                                                    C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                                    let mut v18 = v18.into_context_iter();
                                                                    while let Some(v19) = v18.next(ctx) {
                                                                        if let &InstructionData::IntCompare {
                                                                            opcode: ref v754,
                                                                            args: ref v755,
                                                                            cond: ref v756,
                                                                        } = &v19.1 {
                                                                            if let &Opcode::Icmp = v754 {
                                                                                match v756 {
                                                                                    &IntCC::Equal => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v774 = constructor_ule(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 360.
                                                                                                        returns.extend(Some(v774));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v775 = constructor_ule(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 362.
                                                                                                        returns.extend(Some(v775));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_176
                                                                                    }
                                                                                    &IntCC::UnsignedGreaterThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.0 {
                                                                                                    if v757.1 == v763.1 {
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 342.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 347.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_177
                                                                                    }
                                                                                    &IntCC::UnsignedLessThan => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v19.0 {
                                                                                                let v757 = C::unpack_value_array_2(ctx, v755);
                                                                                                let v763 = C::unpack_value_array_2(ctx, v761);
                                                                                                if v757.0 == v763.1 {
                                                                                                    if v757.1 == v763.0 {
                                                                                                        let v766 = constructor_ne(ctx, v2.0, v757.0, v757.1);
                                                                                                        // Rule at src/opts/bitops.isle line 343.
                                                                                                        returns.extend(Some(v766));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                        let v767 = constructor_ne(ctx, v2.0, v757.1, v757.0);
                                                                                                        // Rule at src/opts/bitops.isle line 344.
                                                                                                        returns.extend(Some(v767));
                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_178
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                false
                                                            }
                                                            )() { return true; } // __isle_arm_175
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                false
                                            }
                                            )() { return true; } // __isle_arm_157
                                        }
                                        &InstructionData::Unary {
                                            opcode: ref v26,
                                            arg: v27,
                                        } => {
                                            if (|| -> bool {
                                                if let &Opcode::Bnot = v26 {
                                                    if v2.0 == v11.0 {
                                                        let mut v18 = C::inst_data_value_etor_returns::default();
                                                        C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                        let mut v18 = v18.into_context_iter();
                                                        while let Some(v19) = v18.next(ctx) {
                                                            match &v19.1 {
                                                                &InstructionData::Binary {
                                                                    opcode: ref v102,
                                                                    args: ref v103,
                                                                } => {
                                                                    if (|| -> bool {
                                                                        match v102 {
                                                                            &Opcode::Band => {
                                                                                if (|| -> bool {
                                                                                    if v2.0 == v19.0 {
                                                                                        let mut v644 = C::inst_data_value_etor_returns::default();
                                                                                        C::inst_data_value_etor(ctx, v27, &mut v644);
                                                                                        let mut v644 = v644.into_context_iter();
                                                                                        while let Some(v645) = v644.next(ctx) {
                                                                                            if let &InstructionData::Binary {
                                                                                                opcode: ref v648,
                                                                                                args: ref v649,
                                                                                            } = &v645.1 {
                                                                                                match v648 {
                                                                                                    &Opcode::Bor => {
                                                                                                        if (|| -> bool {
                                                                                                            if v2.0 == v645.0 {
                                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                                let v650 = C::unpack_value_array_2(ctx, v649);
                                                                                                                if v104.0 == v650.0 {
                                                                                                                    if v104.1 == v650.1 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v653 = constructor_bnot(ctx, v2.0, v243);
                                                                                                                        // Rule at src/opts/bitops.isle line 671.
                                                                                                                        returns.extend(Some(v653));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v667 = constructor_bnot(ctx, v2.0, v244);
                                                                                                                        // Rule at src/opts/bitops.isle line 677.
                                                                                                                        returns.extend(Some(v667));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                    let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                                    let mut v114 = v114.into_context_iter();
                                                                                                                    while let Some(v115) = v114.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v118,
                                                                                                                            arg: v119,
                                                                                                                        } = &v115.1 {
                                                                                                                            if let &Opcode::Bnot = v118 {
                                                                                                                                if v119 == v650.1 {
                                                                                                                                    if v2.0 == v115.0 {
                                                                                                                                        let v931 = constructor_bnot(ctx, v2.0, v119);
                                                                                                                                        // Rule at src/opts/bitops.isle line 657.
                                                                                                                                        returns.extend(Some(v931));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v104.0 == v650.1 {
                                                                                                                    if v104.1 == v650.0 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v653 = constructor_bnot(ctx, v2.0, v243);
                                                                                                                        // Rule at src/opts/bitops.isle line 673.
                                                                                                                        returns.extend(Some(v653));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v667 = constructor_bnot(ctx, v2.0, v244);
                                                                                                                        // Rule at src/opts/bitops.isle line 675.
                                                                                                                        returns.extend(Some(v667));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                    let mut v114 = C::inst_data_value_etor_returns::default();
                                                                                                                    C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                                                                    let mut v114 = v114.into_context_iter();
                                                                                                                    while let Some(v115) = v114.next(ctx) {
                                                                                                                        if let &InstructionData::Unary {
                                                                                                                            opcode: ref v118,
                                                                                                                            arg: v119,
                                                                                                                        } = &v115.1 {
                                                                                                                            if let &Opcode::Bnot = v118 {
                                                                                                                                if v119 == v650.0 {
                                                                                                                                    if v2.0 == v115.0 {
                                                                                                                                        let v931 = constructor_bnot(ctx, v2.0, v119);
                                                                                                                                        // Rule at src/opts/bitops.isle line 655.
                                                                                                                                        returns.extend(Some(v931));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                let mut v107 = C::inst_data_value_etor_returns::default();
                                                                                                                C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                                                                let mut v107 = v107.into_context_iter();
                                                                                                                while let Some(v108) = v107.next(ctx) {
                                                                                                                    if let &InstructionData::Unary {
                                                                                                                        opcode: ref v111,
                                                                                                                        arg: v112,
                                                                                                                    } = &v108.1 {
                                                                                                                        if let &Opcode::Bnot = v111 {
                                                                                                                            if v2.0 == v108.0 {
                                                                                                                                if v112 == v650.0 {
                                                                                                                                    if v104.1 == v650.1 {
                                                                                                                                        let v930 = constructor_bnot(ctx, v2.0, v112);
                                                                                                                                        // Rule at src/opts/bitops.isle line 651.
                                                                                                                                        returns.extend(Some(v930));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                if v112 == v650.1 {
                                                                                                                                    if v104.1 == v650.0 {
                                                                                                                                        let v930 = constructor_bnot(ctx, v2.0, v112);
                                                                                                                                        // Rule at src/opts/bitops.isle line 653.
                                                                                                                                        returns.extend(Some(v930));
                                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            false
                                                                                                        }
                                                                                                        )() { return true; } // __isle_arm_182
                                                                                                    }
                                                                                                    &Opcode::Bxor => {
                                                                                                        if (|| -> bool {
                                                                                                            if v2.0 == v645.0 {
                                                                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                                                let v650 = C::unpack_value_array_2(ctx, v649);
                                                                                                                if v104.0 == v650.0 {
                                                                                                                    if v104.1 == v650.1 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v653 = constructor_bnot(ctx, v2.0, v243);
                                                                                                                        // Rule at src/opts/bitops.isle line 229.
                                                                                                                        returns.extend(Some(v653));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v667 = constructor_bnot(ctx, v2.0, v244);
                                                                                                                        // Rule at src/opts/bitops.isle line 235.
                                                                                                                        returns.extend(Some(v667));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                                if v104.0 == v650.1 {
                                                                                                                    if v104.1 == v650.0 {
                                                                                                                        let v243 = constructor_bxor(ctx, v2.0, v104.0, v104.1);
                                                                                                                        let v653 = constructor_bnot(ctx, v2.0, v243);
                                                                                                                        // Rule at src/opts/bitops.isle line 231.
                                                                                                                        returns.extend(Some(v653));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                        let v244 = constructor_bxor(ctx, v2.0, v104.1, v104.0);
                                                                                                                        let v667 = constructor_bnot(ctx, v2.0, v244);
                                                                                                                        // Rule at src/opts/bitops.isle line 233.
                                                                                                                        returns.extend(Some(v667));
                                                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            false
                                                                                                        }
                                                                                                        )() { return true; } // __isle_arm_183
                                                                                                    }
                                                                                                    _ => {}
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                        if v27 == v104.0 {
                                                                                            let v638 = constructor_bnot(ctx, v2.0, v104.0);
                                                                                            let v639 = constructor_bor(ctx, v2.0, v104.1, v638);
                                                                                            // Rule at src/opts/bitops.isle line 220.
                                                                                            returns.extend(Some(v639));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                        if v27 == v104.1 {
                                                                                            let v354 = constructor_bor(ctx, v2.0, v104.0, v7.1);
                                                                                            // Rule at src/opts/bitops.isle line 44.
                                                                                            returns.extend(Some(v354));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                    }
                                                                                    false
                                                                                }
                                                                                )() { return true; } // __isle_arm_181
                                                                            }
                                                                            &Opcode::Bxor => {
                                                                                if (|| -> bool {
                                                                                    if v2.0 == v19.0 {
                                                                                        let v104 = C::unpack_value_array_2(ctx, v103);
                                                                                        if v27 == v104.0 {
                                                                                            let v247 = constructor_band(ctx, v2.0, v104.1, v104.0);
                                                                                            let v832 = constructor_bnot(ctx, v2.0, v247);
                                                                                            // Rule at src/opts/bitops.isle line 486.
                                                                                            returns.extend(Some(v832));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                        if v27 == v104.1 {
                                                                                            let v143 = constructor_band(ctx, v2.0, v104.0, v104.1);
                                                                                            let v829 = constructor_bnot(ctx, v2.0, v143);
                                                                                            // Rule at src/opts/bitops.isle line 484.
                                                                                            returns.extend(Some(v829));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                    }
                                                                                    false
                                                                                }
                                                                                )() { return true; } // __isle_arm_184
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        false
                                                                    }
                                                                    )() { return true; } // __isle_arm_180
                                                                }
                                                                &InstructionData::Unary {
                                                                    opcode: ref v29,
                                                                    arg: v30,
                                                                } => {
                                                                    if let &Opcode::Bnot = v29 {
                                                                        if v2.0 == v19.0 {
                                                                            let v709 = constructor_band(ctx, v2.0, v30, v27);
                                                                            let v710 = constructor_bnot(ctx, v2.0, v709);
                                                                            // Rule at src/opts/bitops.isle line 289.
                                                                            returns.extend(Some(v710));
                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                            let v711 = constructor_band(ctx, v2.0, v27, v30);
                                                                            let v712 = constructor_bnot(ctx, v2.0, v711);
                                                                            // Rule at src/opts/bitops.isle line 290.
                                                                            returns.extend(Some(v712));
                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        let mut v644 = C::inst_data_value_etor_returns::default();
                                                        C::inst_data_value_etor(ctx, v27, &mut v644);
                                                        let mut v644 = v644.into_context_iter();
                                                        while let Some(v645) = v644.next(ctx) {
                                                            if let &InstructionData::Binary {
                                                                opcode: ref v648,
                                                                args: ref v649,
                                                            } = &v645.1 {
                                                                match v648 {
                                                                    &Opcode::Bor => {
                                                                        if (|| -> bool {
                                                                            if v2.0 == v645.0 {
                                                                                let v650 = C::unpack_value_array_2(ctx, v649);
                                                                                if v7.0 == v650.0 {
                                                                                    let v741 = constructor_bnot(ctx, v2.0, v650.1);
                                                                                    let v742 = constructor_bor(ctx, v2.0, v7.0, v741);
                                                                                    // Rule at src/opts/bitops.isle line 315.
                                                                                    returns.extend(Some(v742));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v7.0 == v650.1 {
                                                                                    let v737 = constructor_bnot(ctx, v2.0, v650.0);
                                                                                    let v738 = constructor_bor(ctx, v2.0, v7.0, v737);
                                                                                    // Rule at src/opts/bitops.isle line 313.
                                                                                    returns.extend(Some(v738));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                            }
                                                                            false
                                                                        }
                                                                        )() { return true; } // __isle_arm_185
                                                                    }
                                                                    &Opcode::Bxor => {
                                                                        if (|| -> bool {
                                                                            if v2.0 == v645.0 {
                                                                                let v650 = C::unpack_value_array_2(ctx, v649);
                                                                                if v7.0 == v650.0 {
                                                                                    let v741 = constructor_bnot(ctx, v2.0, v650.1);
                                                                                    let v742 = constructor_bor(ctx, v2.0, v7.0, v741);
                                                                                    // Rule at src/opts/bitops.isle line 590.
                                                                                    returns.extend(Some(v742));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v7.0 == v650.1 {
                                                                                    let v737 = constructor_bnot(ctx, v2.0, v650.0);
                                                                                    let v738 = constructor_bor(ctx, v2.0, v7.0, v737);
                                                                                    // Rule at src/opts/bitops.isle line 592.
                                                                                    returns.extend(Some(v738));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                            }
                                                                            false
                                                                        }
                                                                        )() { return true; } // __isle_arm_186
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                    let v348 = C::ty_int_vec128(ctx, v2.0);
                                                    if let Some(v349) = v348 {
                                                        if v11.0 == v349 {
                                                            if v7.0 == v27 {
                                                                let v350 = constructor_iconst_s(ctx, v349, -1_i64);
                                                                let v351 = C::subsume(ctx, v350);
                                                                // Rule at src/opts/bitops.isle line 24.
                                                                returns.extend(Some(v351));
                                                                if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                            }
                                                        }
                                                    }
                                                }
                                                false
                                            }
                                            )() { return true; } // __isle_arm_179
                                        }
                                        &InstructionData::UnaryImm {
                                            opcode: ref v14,
                                            imm: v15,
                                        } => {
                                            if (|| -> bool {
                                                if let &Opcode::Iconst = v14 {
                                                    if v2.0 == v11.0 {
                                                        let v16 = C::u64_from_imm64(ctx, v15);
                                                        if v16 == 0x0_u64 {
                                                            let v17 = C::subsume(ctx, v7.0);
                                                            // Rule at src/opts/bitops.isle line 3.
                                                            returns.extend(Some(v17));
                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                        }
                                                        let mut v18 = C::inst_data_value_etor_returns::default();
                                                        C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                                        let mut v18 = v18.into_context_iter();
                                                        while let Some(v19) = v18.next(ctx) {
                                                            if let &InstructionData::Binary {
                                                                opcode: ref v102,
                                                                args: ref v103,
                                                            } = &v19.1 {
                                                                if let &Opcode::Band = v102 {
                                                                    if v2.0 == v19.0 {
                                                                        let v104 = C::unpack_value_array_2(ctx, v103);
                                                                        let mut v114 = C::inst_data_value_etor_returns::default();
                                                                        C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                        let mut v114 = v114.into_context_iter();
                                                                        while let Some(v115) = v114.next(ctx) {
                                                                            if let &InstructionData::UnaryImm {
                                                                                opcode: ref v132,
                                                                                imm: v133,
                                                                            } = &v115.1 {
                                                                                if let &Opcode::Iconst = v132 {
                                                                                    let v357 = C::ty_mask(ctx, v2.0);
                                                                                    let v358 = C::u64_and(ctx, v357, v16);
                                                                                    let v356 = C::u64_from_imm64(ctx, v133);
                                                                                    let v359 = C::u64_not(ctx, v356);
                                                                                    let v360 = C::u64_and(ctx, v357, v359);
                                                                                    let v361 = C::u64_eq(ctx, v358, v360);
                                                                                    if v361 == true {
                                                                                        if v2.0 == v115.0 {
                                                                                            let v354 = constructor_bor(ctx, v2.0, v104.0, v7.1);
                                                                                            // Rule at src/opts/bitops.isle line 64.
                                                                                            returns.extend(Some(v354));
                                                                                            if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                false
                                            }
                                            )() { return true; } // __isle_arm_187
                                        }
                                        _ => {}
                                    }
                                }
                                let mut v18 = C::inst_data_value_etor_returns::default();
                                C::inst_data_value_etor(ctx, v7.0, &mut v18);
                                let mut v18 = v18.into_context_iter();
                                while let Some(v19) = v18.next(ctx) {
                                    match &v19.1 {
                                        &InstructionData::Binary {
                                            opcode: ref v102,
                                            args: ref v103,
                                        } => {
                                            if (|| -> bool {
                                                match v102 {
                                                    &Opcode::Band => {
                                                        if (|| -> bool {
                                                            if v2.0 == v19.0 {
                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                let mut v107 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                let mut v107 = v107.into_context_iter();
                                                                while let Some(v108) = v107.next(ctx) {
                                                                    match &v108.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v289,
                                                                            args: ref v290,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v289 {
                                                                                    &Opcode::Bor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v108.0 {
                                                                                                let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                if v7.1 == v291.0 {
                                                                                                    let v839 = constructor_band(ctx, v2.0, v104.1, v291.1);
                                                                                                    let v840 = constructor_bor(ctx, v2.0, v839, v291.0);
                                                                                                    // Rule at src/opts/bitops.isle line 498.
                                                                                                    returns.extend(Some(v840));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v291.1 {
                                                                                                    let v847 = constructor_band(ctx, v2.0, v104.1, v291.0);
                                                                                                    let v848 = constructor_bor(ctx, v2.0, v847, v291.1);
                                                                                                    // Rule at src/opts/bitops.isle line 502.
                                                                                                    returns.extend(Some(v848));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_191
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v108.0 {
                                                                                                let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                if v7.1 == v291.0 {
                                                                                                    let v809 = constructor_band(ctx, v2.0, v291.1, v104.1);
                                                                                                    let v810 = constructor_bor(ctx, v2.0, v809, v291.0);
                                                                                                    // Rule at src/opts/bitops.isle line 438.
                                                                                                    returns.extend(Some(v810));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v291.1 {
                                                                                                    let v801 = constructor_band(ctx, v2.0, v291.0, v104.1);
                                                                                                    let v802 = constructor_bor(ctx, v2.0, v801, v291.1);
                                                                                                    // Rule at src/opts/bitops.isle line 434.
                                                                                                    returns.extend(Some(v802));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_192
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_190
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v111,
                                                                            arg: v112,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v111 {
                                                                                if v7.1 == v112 {
                                                                                    if v2.0 == v108.0 {
                                                                                        let v871 = constructor_bor(ctx, v2.0, v112, v104.1);
                                                                                        // Rule at src/opts/bitops.isle line 533.
                                                                                        returns.extend(Some(v871));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                let mut v114 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                let mut v114 = v114.into_context_iter();
                                                                while let Some(v115) = v114.next(ctx) {
                                                                    match &v115.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v273,
                                                                            args: ref v274,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v273 {
                                                                                    &Opcode::Bor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v115.0 {
                                                                                                let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                if v7.1 == v275.0 {
                                                                                                    let v835 = constructor_band(ctx, v2.0, v104.0, v275.1);
                                                                                                    let v836 = constructor_bor(ctx, v2.0, v835, v275.0);
                                                                                                    // Rule at src/opts/bitops.isle line 496.
                                                                                                    returns.extend(Some(v836));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v275.1 {
                                                                                                    let v843 = constructor_band(ctx, v2.0, v104.0, v275.0);
                                                                                                    let v844 = constructor_bor(ctx, v2.0, v843, v275.1);
                                                                                                    // Rule at src/opts/bitops.isle line 500.
                                                                                                    returns.extend(Some(v844));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_194
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v115.0 {
                                                                                                let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                if v7.1 == v275.0 {
                                                                                                    let v813 = constructor_band(ctx, v2.0, v275.1, v104.0);
                                                                                                    let v814 = constructor_bor(ctx, v2.0, v813, v275.0);
                                                                                                    // Rule at src/opts/bitops.isle line 440.
                                                                                                    returns.extend(Some(v814));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v275.1 {
                                                                                                    let v805 = constructor_band(ctx, v2.0, v275.0, v104.0);
                                                                                                    let v806 = constructor_bor(ctx, v2.0, v805, v275.1);
                                                                                                    // Rule at src/opts/bitops.isle line 436.
                                                                                                    returns.extend(Some(v806));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_195
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_193
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v118,
                                                                            arg: v119,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v118 {
                                                                                if v7.1 == v119 {
                                                                                    if v2.0 == v115.0 {
                                                                                        let v872 = constructor_bor(ctx, v2.0, v119, v104.0);
                                                                                        // Rule at src/opts/bitops.isle line 535.
                                                                                        returns.extend(Some(v872));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                if v7.1 == v104.0 {
                                                                    // Rule at src/opts/bitops.isle line 212.
                                                                    returns.extend(Some(v104.0));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                            }
                                                            false
                                                        }
                                                        )() { return true; } // __isle_arm_189
                                                    }
                                                    &Opcode::Bor => {
                                                        if (|| -> bool {
                                                            if v2.0 == v19.0 {
                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                if v7.1 == v104.0 {
                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                    // Rule at src/opts/bitops.isle line 187.
                                                                    returns.extend(Some(v245));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                                if v7.1 == v104.1 {
                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                    // Rule at src/opts/bitops.isle line 188.
                                                                    returns.extend(Some(v245));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                            }
                                                            false
                                                        }
                                                        )() { return true; } // __isle_arm_196
                                                    }
                                                    &Opcode::Bxor => {
                                                        if (|| -> bool {
                                                            if v2.0 == v19.0 {
                                                                let v104 = C::unpack_value_array_2(ctx, v103);
                                                                let mut v107 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v104.0, &mut v107);
                                                                let mut v107 = v107.into_context_iter();
                                                                while let Some(v108) = v107.next(ctx) {
                                                                    match &v108.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v289,
                                                                            args: ref v290,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v289 {
                                                                                    &Opcode::Band => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v108.0 {
                                                                                                let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                if v7.1 == v291.0 {
                                                                                                    let v863 = constructor_bor(ctx, v2.0, v104.1, v291.0);
                                                                                                    // Rule at src/opts/bitops.isle line 518.
                                                                                                    returns.extend(Some(v863));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v291.1 {
                                                                                                    let v855 = constructor_bor(ctx, v2.0, v104.1, v291.1);
                                                                                                    // Rule at src/opts/bitops.isle line 522.
                                                                                                    returns.extend(Some(v855));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_199
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v108.0 {
                                                                                                let v291 = C::unpack_value_array_2(ctx, v290);
                                                                                                if v7.1 == v291.0 {
                                                                                                    let v713 = constructor_bxor(ctx, v2.0, v291.1, v104.1);
                                                                                                    let v714 = constructor_bor(ctx, v2.0, v713, v291.0);
                                                                                                    // Rule at src/opts/bitops.isle line 293.
                                                                                                    returns.extend(Some(v714));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v291.1 {
                                                                                                    let v721 = constructor_bxor(ctx, v2.0, v291.0, v104.1);
                                                                                                    let v722 = constructor_bor(ctx, v2.0, v721, v291.1);
                                                                                                    // Rule at src/opts/bitops.isle line 297.
                                                                                                    returns.extend(Some(v722));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_200
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_198
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v111,
                                                                            arg: v112,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v111 {
                                                                                if v7.1 == v112 {
                                                                                    if v2.0 == v108.0 {
                                                                                        let v668 = constructor_bnot(ctx, v2.0, v104.1);
                                                                                        let v890 = constructor_bor(ctx, v2.0, v112, v668);
                                                                                        // Rule at src/opts/bitops.isle line 597.
                                                                                        returns.extend(Some(v890));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                let mut v114 = C::inst_data_value_etor_returns::default();
                                                                C::inst_data_value_etor(ctx, v104.1, &mut v114);
                                                                let mut v114 = v114.into_context_iter();
                                                                while let Some(v115) = v114.next(ctx) {
                                                                    match &v115.1 {
                                                                        &InstructionData::Binary {
                                                                            opcode: ref v273,
                                                                            args: ref v274,
                                                                        } => {
                                                                            if (|| -> bool {
                                                                                match v273 {
                                                                                    &Opcode::Band => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v115.0 {
                                                                                                let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                if v7.1 == v275.0 {
                                                                                                    let v859 = constructor_bor(ctx, v2.0, v104.0, v275.0);
                                                                                                    // Rule at src/opts/bitops.isle line 516.
                                                                                                    returns.extend(Some(v859));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v275.1 {
                                                                                                    let v851 = constructor_bor(ctx, v2.0, v104.0, v275.1);
                                                                                                    // Rule at src/opts/bitops.isle line 520.
                                                                                                    returns.extend(Some(v851));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_202
                                                                                    }
                                                                                    &Opcode::Bxor => {
                                                                                        if (|| -> bool {
                                                                                            if v2.0 == v115.0 {
                                                                                                let v275 = C::unpack_value_array_2(ctx, v274);
                                                                                                if v7.1 == v275.0 {
                                                                                                    let v717 = constructor_bxor(ctx, v2.0, v275.1, v104.0);
                                                                                                    let v718 = constructor_bor(ctx, v2.0, v717, v275.0);
                                                                                                    // Rule at src/opts/bitops.isle line 295.
                                                                                                    returns.extend(Some(v718));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                                if v7.1 == v275.1 {
                                                                                                    let v725 = constructor_bxor(ctx, v2.0, v275.0, v104.0);
                                                                                                    let v726 = constructor_bor(ctx, v2.0, v725, v275.1);
                                                                                                    // Rule at src/opts/bitops.isle line 299.
                                                                                                    returns.extend(Some(v726));
                                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                                }
                                                                                            }
                                                                                            false
                                                                                        }
                                                                                        )() { return true; } // __isle_arm_203
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                false
                                                                            }
                                                                            )() { return true; } // __isle_arm_201
                                                                        }
                                                                        &InstructionData::Unary {
                                                                            opcode: ref v118,
                                                                            arg: v119,
                                                                        } => {
                                                                            if let &Opcode::Bnot = v118 {
                                                                                if v7.1 == v119 {
                                                                                    if v2.0 == v115.0 {
                                                                                        let v638 = constructor_bnot(ctx, v2.0, v104.0);
                                                                                        let v892 = constructor_bor(ctx, v2.0, v119, v638);
                                                                                        // Rule at src/opts/bitops.isle line 599.
                                                                                        returns.extend(Some(v892));
                                                                                        if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                if v7.1 == v104.0 {
                                                                    let v245 = constructor_bor(ctx, v2.0, v104.0, v104.1);
                                                                    // Rule at src/opts/bitops.isle line 405.
                                                                    returns.extend(Some(v245));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                                if v7.1 == v104.1 {
                                                                    let v246 = constructor_bor(ctx, v2.0, v104.1, v104.0);
                                                                    // Rule at src/opts/bitops.isle line 407.
                                                                    returns.extend(Some(v246));
                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                }
                                                            }
                                                            false
                                                        }
                                                        )() { return true; } // __isle_arm_197
                                                    }
                                                    _ => {}
                                                }
                                                false
                                            }
                                            )() { return true; } // __isle_arm_188
                                        }
                                        &InstructionData::Unary {
                                            opcode: ref v29,
                                            arg: v30,
                                        } => {
                                            if (|| -> bool {
                                                if let &Opcode::Bnot = v29 {
                                                    if v2.0 == v19.0 {
                                                        let mut v654 = C::inst_data_value_etor_returns::default();
                                                        C::inst_data_value_etor(ctx, v30, &mut v654);
                                                        let mut v654 = v654.into_context_iter();
                                                        while let Some(v655) = v654.next(ctx) {
                                                            if let &InstructionData::Binary {
                                                                opcode: ref v658,
                                                                args: ref v659,
                                                            } = &v655.1 {
                                                                match v658 {
                                                                    &Opcode::Bor => {
                                                                        if (|| -> bool {
                                                                            if v2.0 == v655.0 {
                                                                                let v660 = C::unpack_value_array_2(ctx, v659);
                                                                                if v7.1 == v660.0 {
                                                                                    let v743 = constructor_bnot(ctx, v2.0, v660.1);
                                                                                    let v744 = constructor_bor(ctx, v2.0, v660.0, v743);
                                                                                    // Rule at src/opts/bitops.isle line 316.
                                                                                    returns.extend(Some(v744));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v7.1 == v660.1 {
                                                                                    let v739 = constructor_bnot(ctx, v2.0, v660.0);
                                                                                    let v740 = constructor_bor(ctx, v2.0, v660.1, v739);
                                                                                    // Rule at src/opts/bitops.isle line 314.
                                                                                    returns.extend(Some(v740));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                            }
                                                                            false
                                                                        }
                                                                        )() { return true; } // __isle_arm_205
                                                                    }
                                                                    &Opcode::Bxor => {
                                                                        if (|| -> bool {
                                                                            if v2.0 == v655.0 {
                                                                                let v660 = C::unpack_value_array_2(ctx, v659);
                                                                                if v7.1 == v660.0 {
                                                                                    let v743 = constructor_bnot(ctx, v2.0, v660.1);
                                                                                    let v744 = constructor_bor(ctx, v2.0, v660.0, v743);
                                                                                    // Rule at src/opts/bitops.isle line 591.
                                                                                    returns.extend(Some(v744));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                                if v7.1 == v660.1 {
                                                                                    let v739 = constructor_bnot(ctx, v2.0, v660.0);
                                                                                    let v740 = constructor_bor(ctx, v2.0, v660.1, v739);
                                                                                    // Rule at src/opts/bitops.isle line 593.
                                                                                    returns.extend(Some(v740));
                                                                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                                                }
                                                                            }
                                                                            false
                                                                        }
                                                                        )() { return true; } // __isle_arm_206
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if v7.1 == v30 {
                                                        let v348 = C::ty_int_vec128(ctx, v2.0);
                                                        if let Some(v349) = v348 {
                                                            if v19.0 == v349 {
                                                                let v350 = constructor_iconst_s(ctx, v349, -1_i64);
                                                                let v351 = C::subsume(ctx, v350);
                                                                // Rule at src/opts/bitops.isle line 25.
                                                                returns.extend(Some(v351));
                                                                if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                                            }
                                                        }
                                                    }
                                                }
                                                false
                                            }
                                            )() { return true; } // __isle_arm_204
                                        }
                                        _ => {}
                                    }
                                }
                                if v7.0 == v7.1 {
                                    let v17 = C::subsume(ctx, v7.0);
                                    // Rule at src/opts/bitops.isle line 7.
                                    returns.extend(Some(v17));
                                    if returns.len() >= MAX_ISLE_RETURNS { return true; }
                                }
                                false
                            }
                            )() { return true; } // __isle_arm_127
                        }
                        _ => {}
                    }
                    false
                }
                )() { return; } // __isle_arm_0
            }
            _ => {}
        }
    }
}
