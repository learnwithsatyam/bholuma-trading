import Image, { type ImageProps } from "next/image";
import { Button } from "@repo/ui/button";
import styles from "./page.module.css";
import CandleChart from "bholuma-components/CandleChart";

type Props = Omit<ImageProps, "src"> & {
  srcLight: string;
  srcDark: string;
};

const ThemeImage = (props: Props) => {
  const { srcLight, srcDark, ...rest } = props;

  return (
    <>
      <Image {...rest} src={srcLight} className="imgLight" />
      <Image {...rest} src={srcDark} className="imgDark" />
    </>
  );
};

export default function Home() {
  return (
    <main className="p-6">
      <h1 className="text-xl font-bold mb-4">Candlestick Chart Demo</h1>
      <div className="w-[70vw] mx-auto">
      <CandleChart />
      </div>
    </main>
  );
}
