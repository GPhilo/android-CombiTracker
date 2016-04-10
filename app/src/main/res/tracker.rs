#pragma version(1)
#pragma rs java_package_name(it.gphilo.combitracker)

rs_allocation input;

float2 __attribute__((kernel)) gradient(float in, uint32_t x, uint32_t y)
{
    uint32_t w = rsAllocationGetDimX(input);
    uint32_t h = rsAllocationGetDimY(input);
    float2 res;

    if(x-1 < 0 || x+1 >= w)
        res.x = 0;
    else
        res.x = 0.5f * (rsGetElementAt_float(input, x+1, y) - rsGetElementAt_float(input, x-1, y));

    if(y-1 < 0 || y+1 >= h)
            res.y = 0;
    else
        res.y = 0.5f * (rsGetElementAt_float(input, x, y+1) - rsGetElementAt_float(input, x, y-1));

    return res;
}