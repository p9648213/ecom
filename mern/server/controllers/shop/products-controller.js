import Product from "../../models/Product.js";

export const getFilteredProducts = async (_req, res) => {
  try {
    const products = await Product.find({});

    res.status(200).json({
      success: true,
      data: products,
    });
  } catch (error) {
    console.log(error);
    res.status(500).json({
      success: false,
      message: "Some error occured",
    });
  }
};
