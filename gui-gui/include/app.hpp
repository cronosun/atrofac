#ifndef APP_HPP
#define APP_HPP

#include <iostream>
#include <memory>
#include <format>

#include <FL/Fl.H>
#include <FL/Fl_Text_Editor.H>
#include <FL/Fl_Double_Window.H>
#include <FL/Fl_Radio_Button.H>
#include <FL/Fl_Toggle_Button.H>
#include <FL/Fl_Value_Slider.H>
#include <FL/Fl_Box.H>

#include "Fl_Ext.hpp"
#include "Fl_Hover_Ext.hpp"
#include "Fl_Win_Ext.hpp"

namespace fc
{
    class CurveEditor : public Fl_Flow
    {
      private:
      public:
        std::unique_ptr<Fl_Hover_Ext<Fl_Toggle_Button>> applied;
        std::unique_ptr<Fl_Ext<Fl_Value_Slider>> sliders[8];
        CurveEditor(std::string name);
    };

    class App : public Fl_Win_Ext<Fl_Double_Window>
    {
      private:
        std::string curr_profile = "windows";
        std::unique_ptr<Fl_Hover_Ext<Fl_Radio_Button>> profiles[4];
        std::unique_ptr<Fl_Ext<Fl_Box>> temps[9];
        std::unique_ptr<CurveEditor> ce;
        std::unique_ptr<CurveEditor> ge;
        std::unique_ptr<Fl_Hover_Ext<Fl_Button>> applied;

      public:
        App(int W, int H, const char* title = nullptr);
        ~App();

        void init();
        void exit();

        int run(int argc, char** argv);
    };

} // namespace fc

#endif // APP_HPP
