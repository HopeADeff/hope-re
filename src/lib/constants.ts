import type { Icon as IconType } from "@lucide/svelte";

import { ClockIcon, GaugeIcon, ZapIcon } from "@lucide/svelte";

import type { ProtectionMenuProps } from "./components";

export const algorithms: {
  value: Exclude<ProtectionMenuProps["algorithm"], undefined>;
  label: string;
  description: string;
  colour: string;
  bgColour: string;
}[] = [
  {
    value: "noise",
    label: "Noise",
    description: "Add adversarial noise",
    colour: "text-blue-500",
    bgColour: "bg-blue-500/10",
  },
  {
    value: "glaze",
    label: "Glaze",
    description: "Style transfer protection",
    colour: "text-purple-500",
    bgColour: "bg-purple-500/10",
  },
  {
    value: "nightshade",
    label: "Nightshade",
    description: "Data poisoning protection",
    colour: "text-red-500",
    bgColour: "bg-red-500/10",
  },
];

export const targetStyles: {
  value: Exclude<ProtectionMenuProps["targetStyle"], undefined>;
  label: string;
  emoji: string;
}[] = [
  { value: "abstract", label: "Abstract", emoji: "🎨" },
  { value: "impressionist", label: "Impressionist", emoji: "📸" },
  { value: "cubist", label: "Cubist", emoji: "🟥" },
  { value: "sketch", label: "Sketch", emoji: "✏️" },
  { value: "watercolor", label: "Watercolor", emoji: "💧" },
];

export const qualityPresets: {
  value: number;
  label: string;
  time: string;
  icon: typeof IconType;
  colour: string;
}[] = [
  {
    value: 0,
    label: "Faster",
    time: "~20 mins",
    icon: ZapIcon,
    colour: "text-emerald-500",
  },
  {
    value: 25,
    label: "Fast",
    time: "~30 mins",
    icon: ZapIcon,
    colour: "text-green-500",
  },
  {
    value: 50,
    label: "DEFAULT",
    time: "~40 mins",
    icon: GaugeIcon,
    colour: "text-blue-500",
  },
  {
    value: 75,
    label: "Slower",
    time: "~80 mins",
    icon: ClockIcon,
    colour: "text-orange-500",
  },
  {
    value: 100,
    label: "Slowest",
    time: "~160 mins",
    icon: ClockIcon,
    colour: "text-red-500",
  },
];
