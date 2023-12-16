import translationData from "@/locales/id.json";
import dayjs from "dayjs";
import { template, crush, boil, first } from "radash";

export function formatNumber(number: number) {
  const formatter = new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
  });
  return formatter.format(number);
}

export const indonesianLocale = new Intl.Locale("id", {
  region: "ID",
  hourCycle: "h24",
  calendar: "gregory",
});

export const formatDateToLocale = (date: string) => {
  return dayjs(date).format("DD-MM-YYYY");
};

export const translate = (
  key: string,
  templateData: { [key: string]: string } = {}
) => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-argument
  console.log("Key", key);
  console.log(templateData);
  return template(crush(translationData)[key], templateData);
};

export const getInitials = (name: string) => {
  const parts = name.split(" ");
  let initials = "";
  for (let i = 0; i < parts.length; i++) {
    if (parts[i].length > 0 && parts[i] !== "") {
      initials += parts[i][0];
    }
  }
  return initials.length > 2 ? initials.substring(0, 2) : initials;
};

export const checkRoleAccess = (currentRole: number, roleAccess: number[]) => {
  return roleAccess.includes(currentRole);
};

export function getExtension(filename: string) {
  const parts = filename.split(".");
  return parts[parts.length - 1];
}


export const formatAccDate = (date: string) => {
  return dayjs(date).format("DD/MM/YYYY");
};

export const formatCurrency = (currency: string | number) => {
  let currencyValue = parseFloat(String(currency));
  // Handle negative values
  const isNegative = currencyValue < 0;
  if (isNegative) {
    currencyValue = Math.abs(currencyValue);
  }


  const integerPart = Math.floor(currencyValue);
  const decimalPart = (currencyValue % 1).toFixed(2).slice(1);



  const formattedCurrency =
    integerPart.toLocaleString("id-ID", {
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }) +
    decimalPart.replace(".", ",");

  // Add parentheses for negative currency
  return isNegative ? `(${formattedCurrency})` : formattedCurrency;
};

export const textDateReport = (dateStart: string, dateEnd: string): string => {
  if (dateStart === dateEnd) {
    return `Periode ${dateStart}`;
  } else {
    return `Dari tanggal ${dateStart} - ${dateEnd}`;
  }
}

export const roundDecimal = (number: number, decimalPlaces: number): string => {
  let numberValue = parseFloat(String(number));
  const isNegative = numberValue < 0;
  if (isNegative) {
    numberValue = Math.abs(numberValue);
  }
  const integerPart = Math.floor(numberValue);
  const decimalPart = (numberValue % 1).toFixed(decimalPlaces).slice(1);

  const formattedNumber =
    integerPart.toLocaleString("id-ID", {
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }) +
    decimalPart.replace(".", ",");


  return isNegative ? `- ${formattedNumber}` : formattedNumber;
}
