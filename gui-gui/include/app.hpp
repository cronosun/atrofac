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
    struct PlotGraph : public Fl_Widget
    {
        int slices_ = 0;
        int* table_ = nullptr;

        int thickness_ = 3;
        int line_style_ = FL_SOLID;

        PlotGraph(int slices, int x, int y, int w, int h);
        ~PlotGraph();
        void update_data();

        int handle(int event) override;
        void draw() override;
    };

    class CurveEditor : public Fl_Flow
    {
      public:
        std::unique_ptr<Fl_Hover_Ext<Fl_Toggle_Button>> applied;
        std::unique_ptr<PlotGraph> curve;
        CurveEditor(std::string name);
    };

    class App : public Fl_Win_Ext<Fl_Double_Window>
    {
      private:
        std::string curr_profile = "windows";
        std::unique_ptr<Fl_Hover_Ext<Fl_Radio_Button>> profiles[4];

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
