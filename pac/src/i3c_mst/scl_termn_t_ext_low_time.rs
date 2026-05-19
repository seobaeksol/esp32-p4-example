#[doc = "Register `SCL_TERMN_T_EXT_LOW_TIME` reader"]
pub type R = crate::R<SclTermnTExtLowTimeSpec>;
#[doc = "Register `SCL_TERMN_T_EXT_LOW_TIME` writer"]
pub type W = crate::W<SclTermnTExtLowTimeSpec>;
#[doc = "Field `REG_I3C_MST_TERMN_T_EXT_LOW_TIME` reader - NA"]
pub type RegI3cMstTermnTExtLowTimeR = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_TERMN_T_EXT_LOW_TIME` writer - NA"]
pub type RegI3cMstTermnTExtLowTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_termn_t_ext_low_time(&self) -> RegI3cMstTermnTExtLowTimeR {
        RegI3cMstTermnTExtLowTimeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_termn_t_ext_low_time(
        &mut self,
    ) -> RegI3cMstTermnTExtLowTimeW<'_, SclTermnTExtLowTimeSpec> {
        RegI3cMstTermnTExtLowTimeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_termn_t_ext_low_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_termn_t_ext_low_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclTermnTExtLowTimeSpec;
impl crate::RegisterSpec for SclTermnTExtLowTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_termn_t_ext_low_time::R`](R) reader structure"]
impl crate::Readable for SclTermnTExtLowTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_termn_t_ext_low_time::W`](W) writer structure"]
impl crate::Writable for SclTermnTExtLowTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_TERMN_T_EXT_LOW_TIME to value 0x02"]
impl crate::Resettable for SclTermnTExtLowTimeSpec {
    const RESET_VALUE: u32 = 0x02;
}
