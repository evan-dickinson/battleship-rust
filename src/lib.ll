; ModuleID = 'lib0-8787f43e282added376259c1adb08b80.rs'
source_filename = "lib0-8787f43e282added376259c1adb08b80.rs"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-darwin"

%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@byte_str.0 = private unnamed_addr constant <{ [9 x i8] }> <{ [9 x i8] c"So fancy!" }>, align 1
@byte_str.1 = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c"No!" }>, align 1
@byte_str.2 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"Bleh" }>, align 1
@byte_str.3 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"A fine hat" }>, align 1

; lib::client::connect
; Function Attrs: norecurse nounwind readnone uwtable
define void @_ZN3lib6client7connect17h583b2ceab2f1672dE() unnamed_addr #0 {
start:
  ret void
}

; lib::network::connect
; Function Attrs: norecurse nounwind readnone uwtable
define void @_ZN3lib7network7connect17h5c6baf31f5d9cc9fE() unnamed_addr #0 {
start:
  ret void
}

; lib::network::server::connect
; Function Attrs: norecurse nounwind readnone uwtable
define void @_ZN3lib7network6server7connect17h2a770d48aba1ce27E() unnamed_addr #0 {
start:
  ret void
}

; lib::hat_cost
; Function Attrs: norecurse nounwind readonly uwtable
define i64 @_ZN3lib8hat_cost17h91a47119446bd593E({ i64, i64 }* noalias nocapture readonly dereferenceable(16) %hat) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %hat, i64 0, i32 0
  %1 = load i64, i64* %0, align 8, !range !0
  %trunc = trunc i64 %1 to i2
  switch i2 %trunc, label %bb4 [
    i2 0, label %bb6
    i2 1, label %bb2
    i2 -2, label %bb3
    i2 -1, label %bb5
  ]

bb2:                                              ; preds = %start
  br label %bb6

bb3:                                              ; preds = %start
  br label %bb6

bb4:                                              ; preds = %start
  unreachable

bb5:                                              ; preds = %start
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %hat, i64 0, i32 1
  %3 = load i64, i64* %2, align 8
  br label %bb6

bb6:                                              ; preds = %start, %bb2, %bb3, %bb5
  %_0.0 = phi i64 [ %3, %bb5 ], [ 0, %bb3 ], [ 5, %bb2 ], [ 500, %start ]
  ret i64 %_0.0
}

; lib::hats_cost
; Function Attrs: nounwind uwtable
define i64 @_ZN3lib9hats_cost17hed6d091b2fc0b319E([0 x { i64, i64 }]* noalias nonnull readonly %hats.0, i64 %hats.1) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = getelementptr inbounds [0 x { i64, i64 }], [0 x { i64, i64 }]* %hats.0, i64 0, i64 %hats.1, i32 0
  %1 = icmp eq i64 %hats.1, 0
  br i1 %1, label %_ZN4core4iter8iterator8Iterator3sum17h743a7e27ea80070eE.exit, label %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hac56a19c1960d289E.exit.lr.ph.i.i.i.i"

"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hac56a19c1960d289E.exit.lr.ph.i.i.i.i": ; preds = %start
  %2 = getelementptr inbounds [0 x { i64, i64 }], [0 x { i64, i64 }]* %hats.0, i64 0, i64 0, i32 0
  br label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i", %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hac56a19c1960d289E.exit.lr.ph.i.i.i.i"
  %accum.03.i.i.i.i = phi i64 [ 0, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hac56a19c1960d289E.exit.lr.ph.i.i.i.i" ], [ %7, %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i" ]
  %self.sroa.0.02.i.i.i.i = phi i64* [ %2, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hac56a19c1960d289E.exit.lr.ph.i.i.i.i" ], [ %3, %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i" ]
  %3 = getelementptr inbounds i64, i64* %self.sroa.0.02.i.i.i.i, i64 2
  %4 = load i64, i64* %self.sroa.0.02.i.i.i.i, align 8, !range !0, !alias.scope !1
  %trunc.i.i.i.i.i.i.i = trunc i64 %4 to i2
  switch i2 %trunc.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i [
    i2 0, label %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i"
    i2 1, label %bb2.i.i.i.i.i.i.i
    i2 -2, label %bb3.i.i.i.i.i.i.i
    i2 -1, label %bb5.i.i.i.i.i.i.i
  ]

bb2.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i
  br label %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i"

bb3.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i
  br label %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i"

bb4.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i
  unreachable

bb5.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i
  %5 = getelementptr inbounds i64, i64* %self.sroa.0.02.i.i.i.i, i64 1
  %6 = load i64, i64* %5, align 8, !alias.scope !1
  br label %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i"

"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i": ; preds = %bb5.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i, %bb6.i.i.i.i
  %_0.0.i.i.i.i.i.i.i = phi i64 [ %6, %bb5.i.i.i.i.i.i.i ], [ 0, %bb3.i.i.i.i.i.i.i ], [ 5, %bb2.i.i.i.i.i.i.i ], [ 500, %bb6.i.i.i.i ]
  %7 = add i64 %_0.0.i.i.i.i.i.i.i, %accum.03.i.i.i.i
  %8 = icmp eq i64* %3, %0
  br i1 %8, label %_ZN4core4iter8iterator8Iterator3sum17h743a7e27ea80070eE.exit, label %bb6.i.i.i.i

_ZN4core4iter8iterator8Iterator3sum17h743a7e27ea80070eE.exit: ; preds = %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i", %start
  %accum.0.lcssa.i.i.i.i = phi i64 [ 0, %start ], [ %7, %"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE.exit.i.i.i.i" ]
  ret i64 %accum.0.lcssa.i.i.i.i
}

; lib::describe_hat
; Function Attrs: norecurse nounwind readnone uwtable
define { [0 x i8]*, i64 } @_ZN3lib12describe_hat17h3f86662cd91bf59eE(i64, i64) unnamed_addr #0 {
start:
  switch i64 %0, label %bb5 [
    i64 0, label %bb6
    i64 1, label %bb2
    i64 2, label %bb3
    i64 3, label %bb4
  ]

bb2:                                              ; preds = %start
  br label %bb6

bb3:                                              ; preds = %start
  br label %bb6

bb4:                                              ; preds = %start
  br label %bb6

bb5:                                              ; preds = %start
  unreachable

bb6:                                              ; preds = %start, %bb2, %bb3, %bb4
  %_0.sroa.0.0 = phi [0 x i8]* [ bitcast (<{ [9 x i8] }>* @byte_str.0 to [0 x i8]*), %bb4 ], [ bitcast (<{ [3 x i8] }>* @byte_str.1 to [0 x i8]*), %bb3 ], [ bitcast (<{ [4 x i8] }>* @byte_str.2 to [0 x i8]*), %bb2 ], [ bitcast (<{ [10 x i8] }>* @byte_str.3 to [0 x i8]*), %start ]
  %_0.sroa.5.0 = phi i64 [ 9, %bb4 ], [ 3, %bb3 ], [ 4, %bb2 ], [ 10, %start ]
  %2 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %_0.sroa.0.0, 0
  %3 = insertvalue { [0 x i8]*, i64 } %2, i64 %_0.sroa.5.0, 1
  ret { [0 x i8]*, i64 } %3
}

; lib::foo
; Function Attrs: nounwind readonly uwtable
define { i32, i32 } @_ZN3lib3foo17h49fe187d1061c62cE() unnamed_addr #3 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
_ZN4core4iter8iterator8Iterator3nth17h40bef332c4e1c718E.exit:
  ret { i32, i32 } { i32 1, i32 6 }
}

declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #4

attributes #0 = { norecurse nounwind readnone uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" }
attributes #1 = { norecurse nounwind readonly uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" }
attributes #2 = { nounwind uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" }
attributes #3 = { nounwind readonly uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" }
attributes #4 = { "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" }

!0 = !{i64 0, i64 4}
!1 = !{!2, !4, !6}
!2 = distinct !{!2, !3, !"_ZN3lib8hat_cost17h91a47119446bd593E: %hat"}
!3 = distinct !{!3, !"_ZN3lib8hat_cost17h91a47119446bd593E"}
!4 = distinct !{!4, !5, !"_ZN4core3ops8function5FnMut8call_mut17h875295f9abb6c17fE: argument 0"}
!5 = distinct !{!5, !"_ZN4core3ops8function5FnMut8call_mut17h875295f9abb6c17fE"}
!6 = distinct !{!6, !7, !"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE: %elt"}
!7 = distinct !{!7, !"_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold28_$u7b$$u7b$closure$u7d$$u7d$17h5de0a075404a428eE"}
