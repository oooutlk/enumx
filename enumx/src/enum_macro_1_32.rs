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
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty ) => { Enum17<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty ) => { Enum18<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty ) => { Enum19<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty ) => { Enum20<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty ) => { Enum21<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty ) => { Enum22<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty ) => { Enum23<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty ) => { Enum24<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty ) => { Enum25<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty ) => { Enum26<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty, $t26:ty ) => { Enum27<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25,$t26> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty, $t26:ty, $t27:ty ) => { Enum28<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25,$t26,$t27> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty, $t26:ty, $t27:ty, $t28:ty ) => { Enum29<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25,$t26,$t27,$t28> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty, $t26:ty, $t27:ty, $t28:ty, $t29:ty ) => { Enum30<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25,$t26,$t27,$t28,$t29> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty, $t26:ty, $t27:ty, $t28:ty, $t29:ty, $t30:ty ) => { Enum31<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25,$t26,$t27,$t28,$t29,$t30> };
    ( $t0:ty, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty, $t17:ty, $t18:ty, $t19:ty, $t20:ty, $t21:ty, $t22:ty, $t23:ty, $t24:ty, $t25:ty, $t26:ty, $t27:ty, $t28:ty, $t29:ty, $t30:ty, $t31:ty ) => { Enum32<$t0,$t1,$t2,$t3,$t4,$t5,$t6,$t7,$t8,$t9,$t10,$t11,$t12,$t13,$t14,$t15,$t16,$t17,$t18,$t19,$t20,$t21,$t22,$t23,$t24,$t25,$t26,$t27,$t28,$t29,$t30,$t31> };
}