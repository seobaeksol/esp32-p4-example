#[doc = "Register `DLL_DB_ST1` reader"]
pub type R = crate::R<DllDbSt1Spec>;
#[doc = "Field `DB_FIFO_CNT_L` reader - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[31:0\\]"]
pub type DbFifoCntLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Debug for DLL FIFO pointer. Use a 64bits shift register to record pointer changes during the debug window. db_fifo_cnt\\[31:0\\]"]
    #[inline(always)]
    pub fn db_fifo_cnt_l(&self) -> DbFifoCntLR {
        DbFifoCntLR::new(self.bits)
    }
}
#[doc = "MSPI DLL debug status1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllDbSt1Spec;
impl crate::RegisterSpec for DllDbSt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_db_st1::R`](R) reader structure"]
impl crate::Readable for DllDbSt1Spec {}
#[doc = "`reset()` method sets DLL_DB_ST1 to value 0"]
impl crate::Resettable for DllDbSt1Spec {}
