import type { Component } from "solid-js";
import "./css/LoadingScreen.css";

export const LodingScreen: Component<{
  isShow: boolean;
}> = (props) => {
  return (
    <div {...(props.isShow ? {} : {class: "none-visible"})}>
      <div class="
        absolute
        inset-0
        flex
        flex-col
        items-center
        justify-center
        
        bg-gray-400
        bg-opacity-50
        
        py-12
        
        cursor-wait
        ">
        <span class="
          loading-text
          font-bold 
          ext-4xl
          
          bg-clip-text
          text-transparent
          bg-gradient-to-r
          from-indigo-600
          to-indigo-400

          select-none
        ">
          <span>N</span>
          <span>o</span>
          <span>w</span>  
          <span>L</span>
          <span>o</span>
          <span>a</span>
          <span>d</span>
          <span>i</span>
          <span>n</span>
          <span>g</span>
          <span>.</span>
          <span>.</span>
          <span>.</span>
        </span>
      </div>
    </div>
  )
}