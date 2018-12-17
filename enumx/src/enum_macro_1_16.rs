#[macro_export]
macro_rules! Enum {
    ( $t0:ty ) => { Enum1<$t0> };
    ( $t0:ty, $t1:ty ) => { Enum2<$t0,$t1> };
    ( $t0:ty, $t1:ty, $t2:ty ) => { Enum3<$t0,$t1,$t2> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty ) => { Enum4<$t0,$t1,$t2,$t3> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty ) => { Enum5<$t0,$t1,$t2,$t3,$t4> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty ) => { Enum6<$t0,$t1,$t2,$t3,$t4,$t5> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty ) => { Enum7<$t0,$t1,$t2,$t3,$t4,$t5,$t6> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty ) => { Enum8<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty ) => { Enum9<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty ) => { Enum10<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty ) => { Enum11<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty ) => { Enum12<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty ) => { Enum13<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty ) => { Enum14<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty ) => { Enum15<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty ) => { Enum16<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15> };
}
