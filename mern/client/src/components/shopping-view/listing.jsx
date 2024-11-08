import { ArrowUpDownIcon } from "lucide-react";
import { Button } from "../ui/button";
import {
  DropdownMenu,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
  DropdownMenuContent,
} from "../ui/dropdown-menu";
import ProductFilter from "./filter";
import { sortOptions } from "@/config";
import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { fetchAllFilteredProducts } from "@/store/shop/product-slice";
import ShoppingProductTile from "./product-tile";

export default function ShoppingListing() {
  const dispatch = useDispatch();
  const { productList } = useSelector((state) => state.shopProducts);

  useEffect(() => {
    dispatch(fetchAllFilteredProducts());
  }, [dispatch]);

  return (
    <div className="gap-6 grid grid-cols-1 md:grid-cols-[300px_1fr] p-4 md:p-6">
      <ProductFilter />
      <div className="bg-background shadow-sm rounded-lg w-full">
        <div className="flex justify-between items-center p-4 border-b">
          <h2 className="font-extrabold text-lg">All Products</h2>
          <div className="flex items-center gap-3">
            <span className="text-muted-foreground">
              {productList.length} Products
            </span>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button
                  variant="outline"
                  size="sm"
                  className="flex items-center gap-1"
                >
                  <ArrowUpDownIcon className="w-4 h-4" />
                  <span>Soft by</span>
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" className="w-[200px]">
                <DropdownMenuRadioGroup>
                  {sortOptions.map((sortItem) => (
                    <DropdownMenuRadioItem key={sortItem.id}>
                      {sortItem.label}
                    </DropdownMenuRadioItem>
                  ))}
                </DropdownMenuRadioGroup>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
        <div className="gap-4 grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 grid-col-1 p-4">
          {productList && productList.length > 0
            ? productList.map((productItem) => (
                <ShoppingProductTile
                  key={productItem.id}
                  product={productItem}
                />
              ))
            : null}
        </div>
      </div>
    </div>
  );
}
