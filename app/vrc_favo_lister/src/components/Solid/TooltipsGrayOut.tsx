import type { Component } from "solid-js";

export const TooltipsGrayOut: Component<{
  tips: string;
  text: string;
}> = (props) => {
  return (
    <span class="relative group">
      {/* ダークモード時の対応を考える */}
      <span
        class="
          bg-black
          text-white
          
          whitespace-nowrap 
          rounded

          text-sm

          px-2
          py-1

          absolute
          top-8
          left-1/2
          -translate-x-3/2

          before:content-['']
          before:absolute
          before:translate-x-1/2
          before:left-0
          
          before:bottom-7
          before:border-8

          before:border-l-[12px] 
          before:border-b-[18px] 
          before:border-r-[12px] 
          before:border-b-black
          before:border-transparent
          
          opacity-0
          group-hover:opacity-100
          transition
          pointer-events-none
      ">
        {props.tips}
      </span>
      <p class="text-gray-400 cursor-not-allowed">{props.text}</p>
    </span>
  )
}