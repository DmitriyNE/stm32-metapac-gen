MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 2048K /* BANK_1_REGION_1 + BANK_1_REGION_2 + BANK_1_REGION_3 + BANK_2_REGION_1 + BANK_2_REGION_2 + BANK_2_REGION_3 */
    RAM   : ORIGIN = 0x20000000, LENGTH =  320K
    OTP   : ORIGIN = 0x1fff7800, LENGTH =  528
}