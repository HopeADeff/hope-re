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
    colour: "text-indigo-600 dark:text-indigo-400",
    bgColour: "bg-indigo-500/10",
  },
  {
    value: "glaze",
    label: "Glaze",
    description: "Style transfer protection",
    colour: "text-violet-600 dark:text-violet-400",
    bgColour: "bg-violet-500/10",
  },
  {
    value: "nightshade",
    label: "Nightshade",
    description: "Data poisoning protection",
    colour: "text-rose-600 dark:text-rose-400",
    bgColour: "bg-rose-500/10",
  },
];

export const glazeStyles: {
  value: Exclude<ProtectionMenuProps["glazeStyle"], undefined>;
  label: string;
  emoji: string;
  description: string;
}[] = [
  { value: "abstract", label: "Abstract", emoji: "\uD83C\uDFA8", description: "Chaotic brushstrokes" },
  { value: "impressionist", label: "Impressionist", emoji: "\uD83D\uDD8C\uFE0F", description: "Gentle light" },
  { value: "cubist", label: "Cubist", emoji: "\uD83D\uDD37", description: "Fragmented forms" },
  { value: "sketch", label: "Sketch", emoji: "\u270F\uFE0F", description: "Loose pencil lines" },
  { value: "watercolor", label: "Watercolor", emoji: "\uD83D\uDCA7", description: "Flowing colors" },
];

export const nightshadeTargets: {
  value: Exclude<ProtectionMenuProps["nightshadeTarget"], undefined>;
  label: string;
  emoji: string;
  description: string;
}[] = [
  { value: "dog", label: "Dog", emoji: "\uD83D\uDC36", description: "Tricks AI into seeing a dog" },
  { value: "cat", label: "Cat", emoji: "\uD83D\uDC31", description: "Tricks AI into seeing a cat" },
  { value: "car", label: "Car", emoji: "\uD83D\uDE97", description: "Tricks AI into seeing a car" },
  { value: "landscape", label: "Landscape", emoji: "\uD83C\uDF04", description: "Tricks AI into seeing a landscape" },
  { value: "person", label: "Person", emoji: "\uD83E\uDDCD", description: "Tricks AI into seeing a person" },
  { value: "building", label: "Building", emoji: "\uD83C\uDFDB\uFE0F", description: "Tricks AI into seeing a building" },
  { value: "food", label: "Food", emoji: "\uD83C\uDF72", description: "Tricks AI into seeing food" },
  { value: "abstract", label: "Abstract", emoji: "\uD83C\uDFA8", description: "Tricks AI into seeing abstract art" },
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
    colour: "text-emerald-600 dark:text-emerald-400",
  },
  {
    value: 25,
    label: "Fast",
    time: "~30 mins",
    icon: ZapIcon,
    colour: "text-teal-600 dark:text-teal-400",
  },
  {
    value: 50,
    label: "DEFAULT",
    time: "~40 mins",
    icon: GaugeIcon,
    colour: "text-sky-600 dark:text-sky-400",
  },
  {
    value: 75,
    label: "Slower",
    time: "~80 mins",
    icon: ClockIcon,
    colour: "text-amber-600 dark:text-amber-400",
  },
  {
    value: 100,
    label: "Slowest",
    time: "~160 mins",
    icon: ClockIcon,
    colour: "text-rose-600 dark:text-rose-400",
  },
];
