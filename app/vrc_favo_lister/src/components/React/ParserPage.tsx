/*
import React from 'react';
import { Grid, GridItem } from "@yamada-ui/react"

function MyButton({ title }: { title: string }) {
    return (
        <button>{title}</button>
    );
}

export default function ParserPage() {
    return (
        <div>
        <h1>Welcome to my app</h1>
        <MyButton title="I'm a button" />
        </div>
    );
}
*/

/**
 * <template>
    <v-container>
      <v-row justify="center">
        <v-col cols="12" sm="10" md="8" lg="8">
          <h1>World Lister</h1>
          <v-btn
            href="https://vrchat.com/home"
            target="_blank"
            >VRChatにログイン</v-btn
          >
          <v-btn
            href="https://vrchat.com/api/1/worlds/favorites?n=200&offset=0"
            target="_blank"
            >APIを呼び出す(前半分)</v-btn
          >
          <v-btn
            href="https://vrchat.com/api/1/worlds/favorites?n=200&offset=200"
            target="_blank"
            >APIを呼び出す(後半分)</v-btn
          >
          <v-textarea
            v-model="worlds"
            label="ここに貼り付け"
            outlined
          ></v-textarea>
          <v-btn @click="parseWorlds">リスト化</v-btn>
          <v-btn @click="downloadToJSON">JSON形式でダウンロード</v-btn>
        </v-col>
  
        <v-col cols="12" sm="10" md="8" lg="8">
          <v-row justify="center">
            <v-col
              v-for="world in worldsList"
              :key="world.id"
              cols="12"
              sm="6"
              md="4"
              lg="4"
            >
              <ParserWorldCard :world="world" />
            </v-col>
          </v-row>
        </v-col>
      </v-row>
    </v-container>
  </template>
  
  <script setup lang="ts">
import { ref } from 'vue';
import type { World } from '../types/world';
import { devLog, downloadJSON, worldsParser } from '../utils';

  const worlds = ref("");
  const worldsList = ref<World[]>([]);
  
  const parseWorlds = () => {
    devLog(worlds.value);
    if (worlds.value.trim() === "") {
      alert("APIから取得した文字列を貼り付けてください");
      return;
    }
    try {
      worldsParser(worldsList.value, worlds.value);
    } catch (error) {
      alert(error);
    }
  };
  
  const downloadToJSON = () => {
    downloadJSON(worldsList.value);
  };
  </script>
 */
