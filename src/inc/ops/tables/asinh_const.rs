use crate::inc::inc::BigFloatInc;

// hyperbolic arcsine polinomial coeefficients
pub(crate) const ASINH_VALUES: [BigFloatInc; 50] = [
  BigFloatInc { sign: -1, e: -44, n: 44, m: [6667, 6666, 6666, 6666, 6666, 6666, 6666, 6666, 6666, 6666, 1666] },
  BigFloatInc { sign: 1, e: -45, n: 44, m: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7500] },
  BigFloatInc { sign: -1, e: -42, n: 41, m: [7143, 4285, 8571, 7142, 4285, 8571, 7142, 4285, 8571, 4642, 4] },
  BigFloatInc { sign: 1, e: -42, n: 41, m: [4444, 4444, 4444, 4444, 4444, 4444, 4444, 4444, 9444, 381, 3] },
  BigFloatInc { sign: -1, e: -42, n: 41, m: [9091, 9090, 9090, 9090, 9090, 9090, 9090, 9090, 1590, 2372, 2] },
  BigFloatInc { sign: 1, e: -42, n: 41, m: [6923, 2307, 769, 6923, 2307, 769, 6923, 2307, 7644, 7352, 1] },
  BigFloatInc { sign: -1, e: -42, n: 41, m: [0, 0, 0, 0, 0, 0, 0, 5000, 8437, 3964, 1] },
  BigFloatInc { sign: 1, e: -42, n: 41, m: [5294, 8823, 4705, 1176, 5294, 8823, 9705, 9613, 8008, 1551, 1] },
  BigFloatInc { sign: -1, e: -46, n: 44, m: [4211, 7368, 7894, 6315, 1052, 6842, 9473, 4078, 2919, 6095, 9761] },
  BigFloatInc { sign: 1, e: -46, n: 44, m: [4762, 6190, 9047, 4761, 6190, 9047, 4761, 6815, 961, 3358, 8390] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [2609, 3478, 1304, 1739, 5652, 869, 8451, 3598, 2587, 3125, 7] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [0, 0, 0, 0, 0, 3750, 6484, 1889, 1031, 4472, 6] },
  BigFloatInc { sign: -1, e: -46, n: 44, m: [1852, 5185, 8518, 1851, 5185, 3518, 4664, 1923, 7084, 376, 5740] },
  BigFloatInc { sign: 1, e: -46, n: 44, m: [9655, 2068, 7586, 4482, 3103, 5129, 1958, 9904, 8231, 3096, 5153] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [4839, 1935, 6774, 8709, 4233, 5990, 961, 6915, 4348, 6601, 4] },
  BigFloatInc { sign: 1, e: -46, n: 44, m: [8182, 8181, 8181, 3181, 1619, 3709, 773, 9363, 9367, 9070, 4240] },
  BigFloatInc { sign: -1, e: -46, n: 44, m: [7143, 4285, 8571, 9642, 5691, 1940, 2363, 7669, 5883, 9645, 3880] },
  BigFloatInc { sign: 1, e: -46, n: 44, m: [270, 7027, 2702, 3395, 7847, 1386, 5454, 5934, 9382, 2053, 3569] },
  BigFloatInc { sign: -1, e: -46, n: 44, m: [8462, 6153, 384, 6274, 2579, 9243, 7453, 3484, 347, 595, 3297] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [9024, 1493, 2517, 9473, 4810, 6935, 306, 9258, 2164, 578, 3] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [9070, 6002, 1174, 7854, 6764, 6787, 9421, 1108, 7840, 8461, 2] },
  BigFloatInc { sign: 1, e: -46, n: 44, m: [3333, 9583, 8098, 9021, 3478, 3744, 9335, 7289, 3820, 8706, 2657] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [6356, 4386, 671, 9652, 759, 9464, 8834, 8246, 4867, 4894, 2] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [1012, 170, 6634, 8278, 883, 8693, 9751, 2111, 9189, 3380, 2] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [5738, 489, 8513, 4198, 20, 542, 1382, 3710, 7397, 2014, 2] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [8089, 2192, 1931, 5530, 8473, 4277, 1674, 2518, 6103, 776, 2] },
  BigFloatInc { sign: -1, e: -46, n: 44, m: [1346, 606, 7574, 5559, 3774, 3930, 6184, 7283, 1627, 336, 1965] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [8489, 2222, 4656, 1046, 9102, 8927, 1274, 6403, 2640, 8622, 1] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [5108, 304, 7571, 5008, 2192, 3865, 4182, 515, 8112, 7680, 1] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [2082, 7314, 8588, 6599, 4448, 20, 1068, 3583, 939, 6816, 1] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [2178, 603, 6889, 5964, 9839, 3572, 4440, 5351, 6327, 6019, 1] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [28, 3751, 7332, 2880, 7051, 3889, 5676, 6122, 1159, 5284, 1] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [3416, 5058, 4884, 6879, 8575, 9446, 1153, 4079, 2089, 4603, 1] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [8192, 7831, 9666, 6664, 9308, 1128, 2534, 7630, 3991, 3971, 1] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [2759, 1136, 9067, 6420, 321, 6844, 1783, 1275, 8695, 3383, 1] },
  BigFloatInc { sign: 1, e: -46, n: 44, m: [3565, 8593, 7130, 5789, 7206, 6142, 5683, 9028, 8762, 6393, 1283] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [2602, 2631, 9177, 9970, 3791, 14, 168, 8500, 2509, 2325, 1] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [9232, 3193, 8093, 313, 6150, 5167, 4392, 6162, 1525, 1847, 1] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [6999, 3509, 1044, 5086, 521, 8114, 2236, 702, 1833, 1399, 1] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [4872, 1781, 5768, 8418, 5986, 2129, 4472, 6591, 7504, 978, 1] },
  BigFloatInc { sign: -1, e: -43, n: 41, m: [1509, 4885, 682, 8903, 935, 610, 2429, 5872, 5412, 583, 1] },
  BigFloatInc { sign: 1, e: -43, n: 41, m: [5217, 9472, 4785, 9563, 5818, 4427, 6276, 9710, 4867, 211, 1] },
  BigFloatInc { sign: -1, e: -47, n: 44, m: [7681, 2281, 5291, 6555, 9654, 848, 9242, 3312, 6983, 7313, 9860] },
  BigFloatInc { sign: 1, e: -47, n: 44, m: [4197, 9563, 8763, 8048, 2537, 5501, 351, 9564, 9742, 6061, 9529] },
  BigFloatInc { sign: -1, e: -44, n: 41, m: [7035, 438, 5806, 6583, 6868, 5414, 3341, 1836, 692, 2166, 9] },
  BigFloatInc { sign: 1, e: -47, n: 44, m: [5497, 4869, 5779, 6090, 5717, 7599, 1400, 7097, 3091, 3742, 8920] },
  BigFloatInc { sign: -1, e: -47, n: 44, m: [9917, 7516, 7299, 9373, 3511, 2892, 6063, 8675, 2465, 6771, 8639] },
  BigFloatInc { sign: 1, e: -47, n: 44, m: [4882, 9642, 9940, 4813, 2746, 5551, 6343, 7120, 1602, 3984, 8373] },
  BigFloatInc { sign: -1, e: -47, n: 44, m: [8494, 6215, 4401, 300, 7076, 5889, 5098, 6701, 2908, 5221, 8120] },
  BigFloatInc { sign: 1, e: -47, n: 44, m: [3802, 1577, 2687, 5119, 5370, 7396, 5938, 2055, 1358, 1225, 7880] }
];
