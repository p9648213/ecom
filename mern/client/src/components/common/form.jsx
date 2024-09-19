import { Label } from "../ui/label";
import { Input } from "../ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { Textarea } from "../ui/textarea";
import { Button } from "../ui/button";

export default function CommonForm({
  formControls,
  formData,
  setFormData,
  onSubmit,
  buttonText,
}) {
  function renderInputsByComponentType(getControlItems) {
    let element = null;
    const value = formData[getControlItems.name] || "";

    switch (getControlItems.componentType) {
      case "input":
        element = (
          <Input
            name={getControlItems.name}
            type={getControlItems.type}
            placeholder={getControlItems.placeholder}
            id={getControlItems.name}
            autoComplete="on"
            value={value}
            onChange={(event) =>
              setFormData({
                ...formData,
                [getControlItems.name]: event.target.value,
              })
            }
          />
        );
        break;
      case "select":
        element = (
          <Select
            onValueChange={(value) =>
              setFormData({
                ...formData,
                [getControlItems.name]: value,
              })
            }
            value={value}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder={getControlItems.placeholder} />
            </SelectTrigger>
            <SelectContent>
              {getControlItems.options && getControlItems.options.length > 0
                ? getControlItems.options.map((optionItem) => (
                    <SelectItem key={optionItem.id} value={optionItem.id}>
                      {optionItem.label}
                    </SelectItem>
                  ))
                : null}
            </SelectContent>
          </Select>
        );
        break;
      case "textarea":
        element = (
          <Textarea
            name={getControlItems.name}
            id={getControlItems.id}
            placeholder={getControlItems.placeholder}
            value={value}
            onChange={(event) =>
              setFormData({
                ...formData,
                [getControlItems.name]: event.target.value,
              })
            }
          />
        );
        break;
      default:
        element = (
          <Input
            name={getControlItems.name}
            type={getControlItems.type}
            placeholder={getControlItems.placeholder}
            id={getControlItems.name}
            value={value}
            onChange={(event) =>
              setFormData({
                ...formData,
                [getControlItems.name]: event.target.value,
              })
            }
          />
        );
        break;
    }
    return element;
  }

  return (
    <form onSubmit={onSubmit}>
      <div className="flex flex-col gap-3">
        {formControls.map((controlItems) => (
          <div key={controlItems.name} className="grid w-full gap-1.5">
            <Label>{controlItems.label}</Label>
            {renderInputsByComponentType(controlItems)}
          </div>
        ))}
      </div>
      <Button type="submit" className="mt-2 w-full">
        {buttonText || "Submit"}
      </Button>
    </form>
  );
}
